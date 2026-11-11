use crate::state::global_state;

/// Native HUD rendering hook injected into Horizon client's render pipeline.
/// Target: `horizon_client::game::native_hooks::Game<'_>::native_render_hud`
pub fn on_render_hud() {
    let state = global_state();
    let config = state.config.read();
    if !config.hud_indicator {
        return;
    }

    let effective = state.effective_protocol();
    let _text = format!("[ViaHorizonPlus: {}]", effective.name());
    // Horizon's HUD text rendering helper is called here when compiling within Horizon
}
