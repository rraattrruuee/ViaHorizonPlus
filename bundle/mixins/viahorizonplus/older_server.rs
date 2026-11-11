// Injected into horizon_client::net::older_server
fn viahorizonplus_older_server(
    address: &str,
) -> core::ops::ControlFlow<Option<horizon_protocol::adapter::ServerVersion>> {
    use horizon_protocol::adapter::ServerVersion;

    // Read the protocol index saved by ViaHorizonPlus
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

    let Some(idx) = index_opt else {
        return core::ops::ControlFlow::Continue(());
    };

    if idx == 0 {
        // Auto-detect mode: let Horizon's standard ping & older_server detection run
        core::ops::ControlFlow::Continue(())
    } else if idx == 1 {
        // Native 26.3: no translation adapter
        core::ops::ControlFlow::Break(None)
    } else {
        const PROTO_NUMS: &[i32] = &[
            0, 777, 776, 775, 774, 773, 772, 771, 770, 769, 768, 767, 766,
            765, 764, 763, 762, 761, 760, 759, 758, 757, 756, 755, 754, 753,
            751, 736, 735, 578, 498, 404, 340, 316, 210, 110, 47, 14,
        ];
        let proto = if idx < PROTO_NUMS.len() { PROTO_NUMS[idx] } else { 776 };
        if proto <= 47 {
            // 1.8.9 or Beta 1.7.3: handled without modern adapter
            core::ops::ControlFlow::Break(None)
        } else if let Some(sv) = ServerVersion::from_protocol(proto) {
            core::ops::ControlFlow::Break(Some(sv))
        } else {
            // Fall back to oldest available modern adapter (1.20.6)
            core::ops::ControlFlow::Break(Some(ServerVersion::V1_20_6))
        }
    }
}
