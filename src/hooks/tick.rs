use crate::state::global_state;

/// Native tick hook injected into Horizon client's game loop.
/// Target: `horizon_client::game::native_hooks::Game<'_>::native_tick`
pub fn on_tick() {
    let _state = global_state();
    // Update any tick-based timers, ping negotiations, or combat/movement tick mechanics
}
