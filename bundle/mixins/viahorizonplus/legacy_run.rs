// Injected into horizon_client::net::run
fn viahorizonplus_net_run(
    address: &str,
    name: &str,
    auth: Option<&AuthSession>,
    info: &cfg::ClientInformation,
    lang: &Language,
    send: &Sender<Event>,
    shared_writer: &Mutex<Option<ConnWriter>>,
    configuring: &AtomicBool,
    shared_counters: &Mutex<Option<Arc<Counters>>>,
    cookies: &Cookies,
    far: &crate::lod_net::Inbox,
    resources: &Path,
    recorder: &Arc<Mutex<Option<StudioRecorder>>>,
) -> core::ops::ControlFlow<Result<Reason, Reason>> {
    let join_mojang_ipv4 = |account: &AuthSession, server_hash: &str| -> Result<(), String> {
        let body = serde_json::json!({
            "accessToken": account.access_token(),
            "selectedProfile": format!("{:032x}", account.uuid),
            "serverId": server_hash,
        }).to_string();

        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(15))
            .resolver(|netloc: &str| {
                use std::net::ToSocketAddrs;
                let addrs: Vec<std::net::SocketAddr> = netloc.to_socket_addrs()?.collect();
                let v4: Vec<std::net::SocketAddr> = addrs.iter().filter(|a| a.is_ipv4()).copied().collect();
                log::info!("[ViaHorizonPlus] Mojang DNS resolution: all={:?}, selected_v4={:?}", addrs, v4);
                if !v4.is_empty() {
                    Ok(v4)
                } else {
                    Ok(addrs)
                }
            })
            .build();

        let resp = agent.post("https://sessionserver.mojang.com/session/minecraft/join")
            .set("Content-Type", "application/json; charset=utf-8")
            .send_string(&body);

        match resp {
            Ok(_) => Ok(()),
            Err(ureq::Error::Status(code, r)) => {
                let msg = r.into_string().unwrap_or_default();
                Err(format!("HTTP {code}: {msg}"))
            }
            Err(ureq::Error::Transport(e)) => Err(format!("Transport: {e}")),
        }
    };

    if address.starts_with(FRIEND) || address.is_empty() {
        return core::ops::ControlFlow::Continue(());
    }

    // Read user-selected protocol index
    let mut index_opt = None;
    if let Ok(content) = std::fs::read_to_string("horizon-mods/.horizon-mod-data/viahorizonplus/protocol") {
        if let Ok(idx) = content.trim().parse::<usize>() {
            index_opt = Some(idx);
        }
    }
    if index_opt.is_none() {
        if let Ok(home) = std::env::var("HOME") {
            let p = std::path::PathBuf::from(home)
                .join(".local/share/horizon/horizon-mods/.horizon-mod-data/viahorizonplus/protocol");
            if let Ok(content) = std::fs::read_to_string(p) {
                if let Ok(idx) = content.trim().parse::<usize>() {
                    index_opt = Some(idx);
                }
            }
        }
    }

    const PROTO_NUMS: &[i32] = &[
        0, 777, 776, 775, 774, 773, 772, 771, 770, 769, 768, 767, 766,
        765, 764, 763, 762, 761, 760, 759, 758, 757, 756, 755, 754, 753,
        751, 736, 735, 578, 498, 404, 340, 316, 210, 110, 47, 14,
    ];

    use horizon_protocol::adapter::ServerVersion;

    let idx = index_opt.unwrap_or(0);
    let (target_version, proto_num) = if idx == 0 {
        // Auto-detect mode via server ping
        let detected = ping(address, std::time::Duration::from_secs(3)).ok().and_then(|(json, _)| {
            let p = i32::try_from(json["version"]["protocol"].as_i64()?).ok()?;
            ServerVersion::from_protocol(p).map(|v| (Some(v), p))
        });
        detected.unwrap_or((None, PROTOCOL_VERSION))
    } else if idx == 1 {
        // Native 26.3
        (None, PROTOCOL_VERSION)
    } else {
        let p = if idx < PROTO_NUMS.len() { PROTO_NUMS[idx] } else { PROTOCOL_VERSION };
        let sv = ServerVersion::from_protocol(p);
        (sv, p)
    };

    let proto_name = target_version.map_or("Native", |v| v.name());
    log::info!("[ViaHorizonPlus] Engaging adapted session for {} using {} (proto {}) with IPv4 Mojang auth", address, proto_name, proto_num);

    let (host, port) = match parse_address(address) {
        Some(target) => target,
        None => {
            return core::ops::ControlFlow::Break(Err(Reason::keyed_arg(
                "horizon.multiplayer.invalid_address",
                lang,
                address,
            )));
        }
    };

    status(send, &tr(lang, "connect.connecting"));
    let clean_host = host.trim_end_matches('.');
    log_connecting(clean_host, port);

    // 1. Connect TCP (with DNS SRV resolution)
    let (mut c, _, _) = match connect_tcp(clean_host, port, false, READ_TIMEOUT) {
        Ok(res) => res,
        Err(e) => return core::ops::ControlFlow::Break(Err(Reason::plain(e))),
    };

    if let Some(version) = target_version {
        if version != ServerVersion::Current {
            c.adapt(version);
            log::info!("[ViaHorizonPlus] Attached {} adapter for {}", version.name(), clean_host);
        } else {
            log::info!("[ViaHorizonPlus] Native 26.3 (Current) selected, no translation adapter needed for {}", clean_host);
        }
    }
    *shared_counters.lock().unwrap() = Some(c.reader.counters.clone());

    let e = |e: horizon_protocol::Error| Reason::plain(connection_lost(e, lang));

    // 2. Send Handshake packet
    let handshake_proto = target_version.map_or(PROTOCOL_VERSION, |v| v.protocol());
    if let Err(err) = c.send(
        &handshake::Serverbound::Intention(handshake::Intention {
            protocol: handshake_proto,
            host: clean_host.to_owned(),
            port,
            intent: handshake::LOGIN,
        })
        .encode(),
    ) {
        return core::ops::ControlFlow::Break(Err(e(err)));
    }

    // 3. Send LoginStart
    let uuid = auth.map_or_else(|| offline_uuid(name), |a| a.uuid);
    if let Err(err) = c.send(
        &login::Serverbound::Hello(login::ServerHello {
            name: name.to_owned(),
            uuid,
        })
        .encode(),
    ) {
        return core::ops::ControlFlow::Break(Err(e(err)));
    }

    // 4. Packet loop for authentication
    let profile = loop {
        let (id, payload) = match c.recv() {
            Ok(p) => p,
            Err(err) => return core::ops::ControlFlow::Break(Err(e(err))),
        };
        log::info!("[ViaHorizonPlus] Login packet received: id=0x{:02x}, len={}", id, payload.len());

        let decoded = match login::Clientbound::decode(id, &payload) {
            Ok(d) => d,
            Err(err) => return core::ops::ControlFlow::Break(Err(e(err))),
        };

        match decoded {
            login::Clientbound::Compression(p) => {
                c.set_compression(p.threshold);
            }
            login::Clientbound::LoginFinished(p) => {
                let _ = c.send(&login::Serverbound::LoginAcknowledged(Empty).encode());
                log::info!("[ViaHorizonPlus] 1.20.6 Login finished for {} ({:032x})!", p.profile.name, p.profile.uuid);
                break p.profile;
            }
            login::Clientbound::Disconnect(p) => {
                return core::ops::ControlFlow::Break(Ok(Reason::component(&p.reason_json, lang)));
            }
            login::Clientbound::CustomQuery(q) => {
                log::info!("[ViaHorizonPlus] Handling CustomQuery id={}, channel={}", q.transaction, q.channel);
                if let Err(err) = c.send(
                    &login::Serverbound::CustomQueryAnswer(login::CustomQueryAnswer {
                        transaction: q.transaction,
                        payload: None,
                    })
                    .encode(),
                ) {
                    return core::ops::ControlFlow::Break(Err(e(err)));
                }
            }
            login::Clientbound::Hello(hello) => {
                status(send, &tr(lang, "connect.authorizing"));
                let key = match RsaPublicKey::from_der(&hello.public_key) {
                    Ok(k) => k,
                    Err(err) => return core::ops::ControlFlow::Break(Err(Reason::plain(format!("Protocol error: {err}")))),
                };
                let secret = match generate_secret() {
                    Ok(s) => s,
                    Err(err) => return core::ops::ControlFlow::Break(Err(e(err))),
                };

                if hello.should_authenticate {
                    let account = match auth {
                        Some(a) => a,
                        None => return core::ops::ControlFlow::Break(Err(Reason::keyed("horizon.multiplayer.not_signed_in", lang))),
                    };

                    let hash = server_hash(&hello.server_id, &secret, &hello.public_key);
                    log::info!("[ViaHorizonPlus] Authenticating 1.20.6 session with Mojang via IPv4 (hash {})", hash);
                    if let Err(err) = join_mojang_ipv4(account, &hash) {
                        log::warn!("[ViaHorizonPlus] IPv4 join failed ({err}), trying standard auth...");
                        if let Err(e2) = crate::auth::join_server(account, &hash) {
                            return core::ops::ControlFlow::Break(Err(Reason::plain(format!("Mojang join failed: {e2:?}"))));
                        }
                    }
                    log::info!("[ViaHorizonPlus] Mojang session validated successfully!");
                }

                status(send, &tr(lang, "connect.encrypting"));
                let enc_secret = match key.encrypt(&secret) {
                    Ok(s) => s,
                    Err(err) => return core::ops::ControlFlow::Break(Err(e(err))),
                };
                let enc_challenge = match key.encrypt(&hello.challenge) {
                    Ok(ch) => ch,
                    Err(err) => return core::ops::ControlFlow::Break(Err(e(err))),
                };

                if let Err(err) = c.send(
                    &login::Serverbound::Key(login::Key {
                        key: enc_secret,
                        challenge: enc_challenge,
                    })
                    .encode(),
                ) {
                    return core::ops::ControlFlow::Break(Err(e(err)));
                }
                c.enable_encryption(&secret);
                log::info!("[ViaHorizonPlus] Encryption enabled for 1.20.6 session!");
            }
            login::Clientbound::CookieRequest(q) => {
                let payload = cookies.lock().unwrap().get(&q.key).cloned();
                let _ = c.send(
                    &login::Serverbound::CookieResponse(login::CookieResponse {
                        key: q.key,
                        payload,
                    })
                    .encode(),
                );
            }
            login::Clientbound::Unknown(..) => {}
        }
    };

    // 5. Configuration phase & transition to play
    status(send, &tr(lang, "connect.joining"));
    log::info!("[ViaHorizonPlus] Sending configuration brand and client information...");
    if let Err(err) = c.send(
        &cfg::Serverbound::CustomPayload(cfg::CustomPayload {
            channel: "minecraft:brand".into(),
            data: brand(),
        })
        .encode(),
    ) {
        return core::ops::ControlFlow::Break(Err(e(err)));
    }

    if proto_num <= 766 {
        // 1.20.6 and older: ClientInformation must ONLY contain 8 fields (no particle_status from 1.21.2+)
        let mut info_1206 = horizon_protocol::buf::Writer::new();
        info_1206.varint(0); // 0 = CLIENT_INFORMATION
        info_1206.string(&info.language);
        info_1206.i8(info.view_distance);
        info_1206.varint(info.chat_visibility);
        info_1206.bool(info.chat_colors);
        info_1206.u8(info.model_customisation);
        info_1206.varint(info.main_hand);
        info_1206.bool(info.text_filtering);
        info_1206.bool(info.allows_listing);
        if let Err(err) = c.send(&info_1206.data) {
            return core::ops::ControlFlow::Break(Err(e(err)));
        }
        log::info!("[ViaHorizonPlus] Legacy ClientInformation sent without particle_status!");
    } else {
        // Modern 26.1+ / 26.3: ClientInformation includes particle_status
        if let Err(err) = c.send(&cfg::Serverbound::ClientInformation(info.clone()).encode()) {
            return core::ops::ControlFlow::Break(Err(e(err)));
        }
        log::info!("[ViaHorizonPlus] Modern ClientInformation sent!");
    }

    let (mut reader, writer) = c.split();
    *shared_writer.lock().unwrap() = Some(writer);

    let mut registries = Dynamic::default();
    let mut content_verified = false;
    let reply = |p: cfg::Serverbound| -> Result<(), Reason> {
        match shared_writer.lock().unwrap().as_mut() {
            Some(w) => w
                .send(&p.encode())
                .map_err(|e| Reason::plain(connection_lost(e, lang))),
            None => Err(Reason::plain("Disconnected")),
        }
    };

    log::info!("[ViaHorizonPlus] Entering 1.20.6 configuration loop...");
    let configure_result = loop {
        let (id, payload) = match reader.recv() {
            Ok(p) => p,
            Err(err) => {
                log::warn!("[ViaHorizonPlus] Connection closed during configuration: {:?}", err);
                break Err(Reason::plain(connection_lost(err, lang)));
            }
        };
        log::info!("[ViaHorizonPlus] Config packet received: id=0x{:02x}, len={}", id, payload.len());
        let decoded = match cfg::Clientbound::decode(id, &payload) {
            Ok(d) => d,
            Err(err) => {
                log::warn!("[ViaHorizonPlus] Config packet decode error for id=0x{:02x}: {:?}", id, err);
                break Err(Reason::plain(connection_lost(err, lang)));
            }
        };

        match decoded {
            cfg::Clientbound::KeepAlive(k) => {
                log::info!("[ViaHorizonPlus] Config KeepAlive id={}", k.id);
                if let Err(err) = reply(cfg::Serverbound::KeepAlive(k)) {
                    break Err(err);
                }
            }
            cfg::Clientbound::CustomPayload(p) if p.channel == crate::content_identity::CHANNEL => {
                let hash = match crate::content_identity::identity(resources) {
                    Ok(h) => h,
                    Err(_) => break Err(Reason::plain(crate::content_identity::MISMATCH.to_string())),
                };
                content_verified = p.data == hash;
                if let Err(err) = reply(cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                    channel: crate::content_identity::CHANNEL.into(),
                    data: hash.to_vec(),
                })) {
                    break Err(err);
                }
            }
            cfg::Clientbound::CustomPayload(p) if p.channel == horizon_protocol::mod_list::CHANNEL => {
                if let Err(err) = reply(cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                    channel: horizon_protocol::mod_list::CHANNEL.into(),
                    data: horizon_protocol::mod_list::encode(&horizon_protocol::mod_list::shared(
                        &horizon_protocol::mod_list::local(),
                    )),
                })) {
                    break Err(err);
                }
            }
            cfg::Clientbound::CustomPayload(p) => {
                log::info!("[ViaHorizonPlus] Config CustomPayload channel={}", p.channel);
                if p.channel == "minecraft:brand"
                    && horizon_protocol::buf::Reader::new(&p.data)
                        .string(32767)
                        .ok()
                        .as_deref()
                        == Some("horizon")
                {
                    let mut w = horizon_protocol::buf::Writer::new();
                    w.string("horizon");
                    if let Err(err) = reply(cfg::Serverbound::CustomPayload(cfg::CustomPayload {
                        channel: "minecraft:brand".into(),
                        data: w.data,
                    })) {
                        break Err(err);
                    }
                }
                receive_brand(&p, send);
            }
            cfg::Clientbound::Ping(p) => {
                if let Err(err) = reply(cfg::Serverbound::Pong(p)) {
                    break Err(err);
                }
            }
            cfg::Clientbound::SelectKnownPacks(p) => {
                log::info!("[ViaHorizonPlus] Handling SelectKnownPacks (count={})", p.packs.len());
                if let Err(err) = reply(cfg::Serverbound::SelectKnownPacks(cfg::SelectKnownPacks {
                    packs: select_known_packs(p.packs),
                })) {
                    break Err(err);
                }
            }
            cfg::Clientbound::UpdateEnabledFeatures(p) => {
                log::info!("[ViaHorizonPlus] Enabled features: {:?}", p.features);
                let _ = send.send(Event::Features(p.features));
            }
            cfg::Clientbound::RegistryData(mut r) => {
                log::info!("[ViaHorizonPlus] RegistryData registry={} (entries={})", r.registry, r.entries.len());
                fill_known_entries(&mut r, resources);
                registries.add(r);
            }
            cfg::Clientbound::UpdateTags(t) => {
                log::info!("[ViaHorizonPlus] UpdateTags (registries={})", t.registries.len());
                registries.add_tags(&t);
            }
            cfg::Clientbound::CodeOfConduct(c) => {
                let _ = send.send(Event::CodeOfConduct(c.text));
            }
            cfg::Clientbound::ResourcePackPush(p) => {
                let _ = send.send(Event::PackPush(p));
            }
            cfg::Clientbound::ResourcePackPop(p) => {
                let _ = send.send(Event::PackPop(p.id));
            }
            cfg::Clientbound::ShowDialog(d) => {
                let _ = send.send(Event::ShowDialog(d.dialog));
            }
            cfg::Clientbound::ClearDialog(_) => {
                let _ = send.send(Event::ClearDialog);
            }
            cfg::Clientbound::CookieRequest(q) => {
                let payload = cookies.lock().unwrap().get(&q.key).cloned();
                if let Err(err) = reply(cfg::Serverbound::CookieResponse(cfg::CookieResponse {
                    key: q.key,
                    payload,
                })) {
                    break Err(err);
                }
            }
            cfg::Clientbound::StoreCookie(p) => {
                cookies.lock().unwrap().insert(p.key, p.payload);
            }
            cfg::Clientbound::ServerLinks(p) => {
                let _ = send.send(Event::ServerLinks(p.links));
            }
            cfg::Clientbound::Transfer(p) => {
                let _ = send.send(Event::Transfer {
                    host: p.host,
                    port: p.port,
                });
                break Err(Reason::keyed("disconnect.transfer", lang));
            }
            cfg::Clientbound::Disconnect(d) => {
                log::error!("[ViaHorizonPlus] Server sent Disconnect during config: {:?}", d.reason);
                break Err(Reason::tag(&d.reason, lang));
            }
            cfg::Clientbound::FinishConfiguration(_) => {
                log::info!("[ViaHorizonPlus] Server FinishConfiguration received! Sending acknowledgment...");
                registries.custom_components = content_verified;
                let mut w = shared_writer.lock().unwrap_or_else(|e| e.into_inner());
                let w = match w.as_mut() {
                    Some(w) => w,
                    None => break Err(Reason::plain("Disconnected")),
                };
                if let Err(err) = w.send(&cfg::Serverbound::FinishConfiguration(Empty).encode()) {
                    break Err(Reason::plain(connection_lost(err, lang)));
                }
                configuring.store(false, Ordering::Relaxed);
                log::info!("[ViaHorizonPlus] Configuration phase SUCCESS! Transitioning to Play!");
                break Ok(registries);
            }
            cfg::Clientbound::ResetChat(_) => {}
            cfg::Clientbound::Unknown(..) => {}
            _ => {}
        }
    };

    let registries = match configure_result {
        Ok(r) => r,
        Err(err) => return core::ops::ControlFlow::Break(Err(err)),
    };

    let _ = send.send(Event::Joined {
        registries: Arc::new(registries),
        uuid: profile.uuid,
        name: profile.name,
    });

    core::ops::ControlFlow::Break(receive_play(
        &mut reader,
        shared_writer,
        configuring,
        lang,
        send,
        cookies,
        far,
        resources,
        recorder,
    ))
}
