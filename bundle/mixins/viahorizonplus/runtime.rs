// ViaHorizonPlus runtime module injected into Horizon client.
use std::cell::RefCell;

pub const ID: &str = "viahorizonplus";
#[allow(dead_code)]
pub const NAME: &str = "ViaHorizonPlus";
#[allow(dead_code)]
pub const VERSION: &str = "0.1.0";
#[allow(dead_code)]
pub const DESCRIPTION: &str =
    "Multi-version protocol selector and legacy gameplay emulation for Horizon (MIT License).";

/// Complete registry of Minecraft versions available in ViaHorizonPlus.
pub const PROTOCOLS: [(&str, i32); 37] = [
    ("Auto-détection (Ping)", 0),
    ("Horizon 26.3 (Natif)", 777),
    ("26.2", 776),
    ("26.1 / 26.1.2", 775),
    ("1.21.11", 774),
    ("1.21.9 / 1.21.10", 773),
    ("1.21.7 / 1.21.8", 772),
    ("1.21.6", 771),
    ("1.21.5", 770),
    ("1.21.4", 769),
    ("1.21.2 / 1.21.3", 768),
    ("1.21 / 1.21.1", 767),
    ("1.20.5 / 1.20.6", 766),
    ("1.20.3 / 1.20.4", 765),
    ("1.20.2", 764),
    ("1.20 / 1.20.1", 763),
    ("1.19.4", 762),
    ("1.19.3", 761),
    ("1.19 / 1.19.2", 760),
    ("1.18.2", 758),
    ("1.17.1", 756),
    ("1.16.4 / 1.16.5", 754),
    ("1.15.2", 578),
    ("1.14.4", 498),
    ("1.13.2", 404),
    ("1.12.2", 340),
    ("1.11.2", 316),
    ("1.10.2", 210),
    ("1.9.4", 110),
    ("1.8.x", 47),
    ("1.7.10", 5),
    ("1.6.4", 78),
    ("1.5.2", 61),
    ("1.4.7", 51),
    ("1.2.5", 29),
    ("1.0.0", 22),
    ("Beta 1.7.3", 14),
];

pub const VISIBLE_ITEMS: usize = 11;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub protocol_index: usize,
    pub sword_blocking: bool,
    pub sneak_height: bool,
    pub block_hit: bool,
    pub disable_cooldown: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            protocol_index: 0, // 0 = Auto-detect
            sword_blocking: true,
            sneak_height: true,
            block_hit: true,
            disable_cooldown: true,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let protocol_index = get("protocol")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .map_or(0, |idx| idx.min(PROTOCOLS.len() - 1));

        let flag = |key: &str| get(key).is_none_or(|v| v.trim() != "false");

        Self {
            protocol_index,
            sword_blocking: flag("sword_blocking"),
            sneak_height: flag("sneak_height"),
            block_hit: flag("block_hit"),
            disable_cooldown: flag("disable_cooldown"),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "OUI" } else { "NON" };
        let (proto_name, proto_id) = PROTOCOLS[self.protocol_index];
        vec![
            if self.protocol_index == 0 {
                format!("Version: {}", proto_name)
            } else {
                format!("Version: {} (v{})", proto_name, proto_id)
            },
            format!("Blocage à l'épée 1.8: {}", on(self.sword_blocking)),
            format!("Hauteur sneak 1.8 (1.54m): {}", on(self.sneak_height)),
            format!("Animation block-hit 1.7/1.8: {}", on(self.block_hit)),
            format!("Désactiver cooldown 1.9+: {}", on(self.disable_cooldown)),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => {
                self.protocol_index = (self.protocol_index + 1) % PROTOCOLS.len();
            }
            1 => self.sword_blocking = !self.sword_blocking,
            2 => self.sneak_height = !self.sneak_height,
            3 => self.block_hit = !self.block_hit,
            4 => self.disable_cooldown = !self.disable_cooldown,
            _ => {}
        }
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        vec![
            ("protocol", self.protocol_index.to_string()),
            ("sword_blocking", self.sword_blocking.to_string()),
            ("sneak_height", self.sneak_height.to_string()),
            ("block_hit", self.block_hit.to_string()),
            ("disable_cooldown", self.disable_cooldown.to_string()),
        ]
    }
}

thread_local! {
    static CURRENT_SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
    static UI_STATE: RefCell<UiState> = const {
        RefCell::new(UiState {
            dropdown_open: false,
            settings_open: false,
            scroll_offset: 0,
        })
    };
}

#[derive(Clone, Copy, Debug)]
struct UiState {
    dropdown_open: bool,
    settings_open: bool,
    scroll_offset: usize,
}

pub fn ensure_loaded(get: impl Fn(&str) -> Option<String>) {
    CURRENT_SETTINGS.with(|s| {
        let mut borrow = s.borrow_mut();
        if borrow.is_none() {
            *borrow = Some(Settings::read(get));
        }
    });
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    CURRENT_SETTINGS.with(|s| {
        let mut borrow = s.borrow_mut();
        if let Some(set) = *borrow {
            set
        } else {
            let loaded = Settings::read(get);
            *borrow = Some(loaded);
            loaded
        }
    })
}

pub fn set_settings(settings: Settings) {
    CURRENT_SETTINGS.with(|s| {
        *s.borrow_mut() = Some(settings);
    });
}

pub fn get_settings() -> Settings {
    CURRENT_SETTINGS.with(|s| s.borrow().unwrap_or_default())
}

/// Called by older_server hook to get the user-selected server version.
#[allow(dead_code)]
pub fn forced_server_version() -> Option<Option<horizon_protocol::adapter::ServerVersion>> {
    let settings = get_settings();
    if settings.protocol_index == 0 {
        // Auto-detect mode: let vanilla ping run!
        None
    } else if settings.protocol_index == 1 {
        // Native Horizon 26.3
        Some(None)
    } else {
        let (_name, proto) = PROTOCOLS[settings.protocol_index];
        use horizon_protocol::adapter::ServerVersion;
        let mapped = ServerVersion::from_protocol(proto);
        if mapped.is_some() {
            Some(mapped)
        } else {
            // For versions older than 1.20.6 (e.g. 1.8), adapt to the oldest available adapter
            Some(Some(ServerVersion::V1_20_6))
        }
    }
}

fn point_in_rect(p: [f32; 2], r: [f32; 4]) -> bool {
    let [px, py] = p;
    let [rx, ry, rw, rh] = r;
    px >= rx && px <= rx + rw && py >= ry && py <= ry + rh
}

// ---------------------------------------------------------------------------
// UI Rendering & Interaction on the Multiplayer Screen
// ---------------------------------------------------------------------------

#[allow(private_interfaces)]
pub(super) fn draw_multiplayer_ui(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
) {
    let settings = get_settings();
    let ui = UI_STATE.with(|u| *u.borrow());

    let (w, h) = (screen.rect[2], screen.rect[3]);
    let mouse = screen.mouse;

    // Header buttons (top right)
    let btn_ver_rect = [w - 195.0, 6.0, 190.0, 20.0];
    let btn_set_rect = [w - 275.0, 6.0, 75.0, 20.0];

    let ver_hover = point_in_rect(mouse, btn_ver_rect);
    let set_hover = point_in_rect(mouse, btn_set_rect);

    // 1. Draw "⚙ Via" button
    let set_bg = if set_hover || ui.settings_open { 0xFF432A78 } else { 0xFF2A2A2A };
    game.native_fill(batch, btn_set_rect, set_bg);
    game.native_fill(batch, [btn_set_rect[0], btn_set_rect[1], btn_set_rect[2], 1.0], 0xFF6A36C2);
    game.native_fill(batch, [btn_set_rect[0], btn_set_rect[1] + btn_set_rect[3] - 1.0, btn_set_rect[2], 1.0], 0xFF111111);
    game.native_text(batch, "⚙ Via", [btn_set_rect[0] + 18.0, btn_set_rect[1] + 6.0], 0xFFFFFF, true);

    // 2. Draw Version Selector button
    let (cur_name, cur_proto) = PROTOCOLS[settings.protocol_index];
    let (ver_label, text_color) = if settings.protocol_index == 0 {
        ("⚡ Auto (Ping) ▼".to_string(), 0xFFAA00)
    } else {
        (format!("🌐 {} (v{}) ▼", cur_name, cur_proto), 0x55FFFF)
    };

    let ver_bg = if ver_hover || ui.dropdown_open { 0xFF432A78 } else { 0xFF2A2A2A };
    game.native_fill(batch, btn_ver_rect, ver_bg);
    game.native_fill(batch, [btn_ver_rect[0], btn_ver_rect[1], btn_ver_rect[2], 1.0], 0xFF6A36C2);
    game.native_fill(batch, [btn_ver_rect[0], btn_ver_rect[1] + btn_ver_rect[3] - 1.0, btn_ver_rect[2], 1.0], 0xFF111111);
    game.native_text(batch, &ver_label, [btn_ver_rect[0] + 8.0, btn_ver_rect[1] + 6.0], text_color, true);

    // 3. Draw Version Dropdown list if opened
    if ui.dropdown_open {
        let item_h = 17.0;
        let menu_w = 210.0;
        let visible_count = VISIBLE_ITEMS.min(PROTOCOLS.len());
        let menu_h = (visible_count as f32) * item_h + 26.0;
        let menu_rect = [w - 215.0, 28.0, menu_w, menu_h];

        // Background shadow & panel
        game.native_fill(batch, [menu_rect[0] - 2.0, menu_rect[1] - 2.0, menu_w + 4.0, menu_h + 4.0], 0x88000000);
        game.native_fill(batch, menu_rect, 0xFA14141C);
        game.native_fill(batch, [menu_rect[0], menu_rect[1], menu_rect[2], 1.5], 0xFF6A36C2);
        game.native_fill(batch, [menu_rect[0], menu_rect[1] + menu_rect[3] - 1.5, menu_rect[2], 1.5], 0xFF6A36C2);
        game.native_fill(batch, [menu_rect[0], menu_rect[1], 1.5, menu_rect[3]], 0xFF6A36C2);
        game.native_fill(batch, [menu_rect[0] + menu_rect[2] - 1.5, menu_rect[1], 1.5, menu_rect[3]], 0xFF6A36C2);

        // Header hint
        let header_text = format!("SÉLECTION DU PROTOCOLE ({}/{})", ui.scroll_offset + 1, PROTOCOLS.len());
        game.native_text(batch, &header_text, [menu_rect[0] + 6.0, menu_rect[1] + 4.0], 0xFFAA00, true);

        // Scroll buttons
        let btn_prev_rect = [menu_rect[0] + menu_w - 42.0, menu_rect[1] + 2.0, 18.0, 14.0];
        let btn_next_rect = [menu_rect[0] + menu_w - 20.0, menu_rect[1] + 2.0, 18.0, 14.0];
        game.native_fill(batch, btn_prev_rect, if point_in_rect(mouse, btn_prev_rect) { 0xFF5B39A8 } else { 0xFF33333E });
        game.native_text(batch, "▲", [btn_prev_rect[0] + 4.0, btn_prev_rect[1] + 2.0], 0xFFFFFF, false);
        game.native_fill(batch, btn_next_rect, if point_in_rect(mouse, btn_next_rect) { 0xFF5B39A8 } else { 0xFF33333E });
        game.native_text(batch, "▼", [btn_next_rect[0] + 4.0, btn_next_rect[1] + 2.0], 0xFFFFFF, false);

        // Render visible items
        for i in 0..visible_count {
            let actual_idx = ui.scroll_offset + i;
            if actual_idx >= PROTOCOLS.len() {
                break;
            }
            let (name, proto) = PROTOCOLS[actual_idx];
            let item_y = menu_rect[1] + 19.0 + (i as f32) * item_h;
            let item_rect = [menu_rect[0] + 3.0, item_y, menu_rect[2] - 6.0, item_h];
            let item_hover = point_in_rect(mouse, item_rect);

            let is_selected = actual_idx == settings.protocol_index;
            if is_selected {
                game.native_fill(batch, item_rect, 0xFF3D256B);
            } else if item_hover {
                game.native_fill(batch, item_rect, 0xFF5B39A8);
            }

            let bullet = if is_selected { "● " } else { "  " };
            let display_name = if actual_idx == 0 {
                format!("{}⚡ {}", bullet, name)
            } else {
                format!("{}{}", bullet, name)
            };

            let name_color = if is_selected {
                0x55FF55
            } else if actual_idx == 0 {
                0xFFAA00
            } else if item_hover {
                0xFFFFFF
            } else {
                0xDDDDDD
            };
            game.native_text(batch, &display_name, [item_rect[0] + 4.0, item_y + 3.0], name_color, true);

            if actual_idx > 0 {
                let proto_text = format!("v{}", proto);
                let proto_w = game.native_text_width(&proto_text);
                let proto_x = item_rect[0] + item_rect[2] - proto_w - 6.0;
                game.native_text(batch, &proto_text, [proto_x, item_y + 3.0], 0xAAAAAA, true);
            }
        }
    }

    // 4. Draw Via Settings Modal if opened
    if ui.settings_open {
        // Backdrop
        game.native_fill(batch, [0.0, 0.0, w, h], 0x88000000);

        let modal_w = 320.0;
        let modal_h = 225.0;
        let modal_x = ((w - modal_w) / 2.0).floor();
        let modal_y = ((h - modal_h) / 2.0).floor();
        let modal_rect = [modal_x, modal_y, modal_w, modal_h];

        // Box & border
        game.native_fill(batch, modal_rect, 0xFB1A1A22);
        game.native_fill(batch, [modal_x, modal_y, modal_w, 2.0], 0xFF6A36C2);
        game.native_fill(batch, [modal_x, modal_y + modal_h - 2.0, modal_w, 2.0], 0xFF6A36C2);
        game.native_fill(batch, [modal_x, modal_y, 2.0, modal_h], 0xFF6A36C2);
        game.native_fill(batch, [modal_x + modal_w - 2.0, modal_y, 2.0, modal_h], 0xFF6A36C2);

        // Header
        game.native_text(batch, "ViaHorizonPlus — Réglages d'émulation", [modal_x + 16.0, modal_y + 12.0], 0xFFAA00, true);

        let on = |b: bool| if b { "OUI" } else { "NON" };

        // Option 1
        let opt1_rect = [modal_x + 16.0, modal_y + 38.0, modal_w - 32.0, 20.0];
        let opt1_bg = if point_in_rect(mouse, opt1_rect) { 0xFF5B39A8 } else { 0xFF2A2A32 };
        game.native_fill(batch, opt1_rect, opt1_bg);
        game.native_text(batch, &format!("Blocage épée 1.8: {}", on(settings.sword_blocking)), [opt1_rect[0] + 10.0, opt1_rect[1] + 6.0], 0xFFFFFF, true);

        // Option 2
        let opt2_rect = [modal_x + 16.0, modal_y + 64.0, modal_w - 32.0, 20.0];
        let opt2_bg = if point_in_rect(mouse, opt2_rect) { 0xFF5B39A8 } else { 0xFF2A2A32 };
        game.native_fill(batch, opt2_rect, opt2_bg);
        game.native_text(batch, &format!("Hauteur sneak 1.8 (1.54m): {}", on(settings.sneak_height)), [opt2_rect[0] + 10.0, opt2_rect[1] + 6.0], 0xFFFFFF, true);

        // Option 3
        let opt3_rect = [modal_x + 16.0, modal_y + 90.0, modal_w - 32.0, 20.0];
        let opt3_bg = if point_in_rect(mouse, opt3_rect) { 0xFF5B39A8 } else { 0xFF2A2A32 };
        game.native_fill(batch, opt3_rect, opt3_bg);
        game.native_text(batch, &format!("Animation block-hit: {}", on(settings.block_hit)), [opt3_rect[0] + 10.0, opt3_rect[1] + 6.0], 0xFFFFFF, true);

        // Option 4
        let opt4_rect = [modal_x + 16.0, modal_y + 116.0, modal_w - 32.0, 20.0];
        let opt4_bg = if point_in_rect(mouse, opt4_rect) { 0xFF5B39A8 } else { 0xFF2A2A32 };
        game.native_fill(batch, opt4_rect, opt4_bg);
        game.native_text(batch, &format!("Désactiver cooldown 1.9+: {}", on(settings.disable_cooldown)), [opt4_rect[0] + 10.0, opt4_rect[1] + 6.0], 0xFFFFFF, true);

        // License notice
        game.native_text(batch, "ViaHorizonPlus — Licence MIT", [modal_x + 18.0, modal_y + 144.0], 0x888888, false);

        // Close button
        let close_rect = [modal_x + (modal_w - 100.0) / 2.0, modal_y + modal_h - 32.0, 100.0, 20.0];
        let close_bg = if point_in_rect(mouse, close_rect) { 0xFF6A36C2 } else { 0xFF432A78 };
        game.native_fill(batch, close_rect, close_bg);
        game.native_text(batch, "Fermer", [close_rect[0] + 28.0, close_rect[1] + 6.0], 0xFFFFFF, true);
    }
}

pub fn on_multiplayer_click(mouse: [f32; 2], w: f32, h: f32) -> bool {
    let mut ui = UI_STATE.with(|u| *u.borrow());
    let mut settings = get_settings();

    // 1. Settings modal interactions
    if ui.settings_open {
        let modal_w = 320.0;
        let modal_h = 210.0;
        let modal_x = ((w - modal_w) / 2.0).floor();
        let modal_y = ((h - modal_h) / 2.0).floor();
        let modal_rect = [modal_x, modal_y, modal_w, modal_h];

        if !point_in_rect(mouse, modal_rect) {
            ui.settings_open = false;
            UI_STATE.with(|u| *u.borrow_mut() = ui);
            return true;
        }

        let opt1_rect = [modal_x + 16.0, modal_y + 38.0, modal_w - 32.0, 20.0];
        let opt2_rect = [modal_x + 16.0, modal_y + 64.0, modal_w - 32.0, 20.0];
        let opt3_rect = [modal_x + 16.0, modal_y + 90.0, modal_w - 32.0, 20.0];
        let opt4_rect = [modal_x + 16.0, modal_y + 116.0, modal_w - 32.0, 20.0];
        let close_rect = [modal_x + (modal_w - 100.0) / 2.0, modal_y + modal_h - 32.0, 100.0, 20.0];

        if point_in_rect(mouse, opt1_rect) {
            settings.sword_blocking = !settings.sword_blocking;
            set_settings(settings);
            return true;
        }
        if point_in_rect(mouse, opt2_rect) {
            settings.sneak_height = !settings.sneak_height;
            set_settings(settings);
            return true;
        }
        if point_in_rect(mouse, opt3_rect) {
            settings.block_hit = !settings.block_hit;
            set_settings(settings);
            return true;
        }
        if point_in_rect(mouse, opt4_rect) {
            settings.disable_cooldown = !settings.disable_cooldown;
            set_settings(settings);
            return true;
        }
        if point_in_rect(mouse, close_rect) {
            ui.settings_open = false;
            UI_STATE.with(|u| *u.borrow_mut() = ui);
            return true;
        }
        return true;
    }

    // 2. Dropdown interactions
    if ui.dropdown_open {
        let menu_w = 210.0;
        let item_h = 17.0;
        let visible_count = VISIBLE_ITEMS.min(PROTOCOLS.len());
        let menu_h = (visible_count as f32) * item_h + 26.0;
        let menu_rect = [w - 215.0, 28.0, menu_w, menu_h];

        if point_in_rect(mouse, menu_rect) {
            // Scroll buttons
            let btn_prev_rect = [menu_rect[0] + menu_w - 42.0, menu_rect[1] + 2.0, 18.0, 14.0];
            let btn_next_rect = [menu_rect[0] + menu_w - 20.0, menu_rect[1] + 2.0, 18.0, 14.0];

            if point_in_rect(mouse, btn_prev_rect) {
                if ui.scroll_offset > 0 {
                    ui.scroll_offset -= 1;
                    UI_STATE.with(|u| *u.borrow_mut() = ui);
                }
                return true;
            }
            if point_in_rect(mouse, btn_next_rect) {
                if ui.scroll_offset + visible_count < PROTOCOLS.len() {
                    ui.scroll_offset += 1;
                    UI_STATE.with(|u| *u.borrow_mut() = ui);
                }
                return true;
            }

            // Clicked item
            let items_y_start = menu_rect[1] + 19.0;
            if mouse[1] >= items_y_start {
                let row = ((mouse[1] - items_y_start) / item_h) as usize;
                let actual_idx = ui.scroll_offset + row;
                if actual_idx < PROTOCOLS.len() {
                    settings.protocol_index = actual_idx;
                    set_settings(settings);
                    ui.dropdown_open = false;
                    UI_STATE.with(|u| *u.borrow_mut() = ui);
                    log::info!("[ViaHorizonPlus] Changed protocol to: {} (proto {})", PROTOCOLS[actual_idx].0, PROTOCOLS[actual_idx].1);
                    return true;
                }
            }
            return true;
        }

        // Click outside dropdown closes it
        ui.dropdown_open = false;
        UI_STATE.with(|u| *u.borrow_mut() = ui);
    }

    // 3. Header buttons
    let btn_ver_rect = [w - 195.0, 6.0, 190.0, 20.0];
    let btn_set_rect = [w - 275.0, 6.0, 75.0, 20.0];

    if point_in_rect(mouse, btn_ver_rect) {
        ui.dropdown_open = !ui.dropdown_open;
        ui.settings_open = false;
        UI_STATE.with(|u| *u.borrow_mut() = ui);
        return true;
    }

    if point_in_rect(mouse, btn_set_rect) {
        ui.settings_open = !ui.settings_open;
        ui.dropdown_open = false;
        UI_STATE.with(|u| *u.borrow_mut() = ui);
        return true;
    }

    false
}

pub fn on_multiplayer_scroll(scroll_y: f64) -> bool {
    let mut ui = UI_STATE.with(|u| *u.borrow());
    if ui.dropdown_open {
        let visible_count = VISIBLE_ITEMS.min(PROTOCOLS.len());
        if scroll_y > 0.0 && ui.scroll_offset > 0 {
            ui.scroll_offset -= 1;
            UI_STATE.with(|u| *u.borrow_mut() = ui);
            return true;
        } else if scroll_y < 0.0 && ui.scroll_offset + visible_count < PROTOCOLS.len() {
            ui.scroll_offset += 1;
            UI_STATE.with(|u| *u.borrow_mut() = ui);
            return true;
        }
    }
    false
}
