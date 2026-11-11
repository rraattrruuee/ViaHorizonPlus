// Render hook for ViaHorizonPlus UI overlay on the Multiplayer screen
fn viahorizonplus_screen_render(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
) {
    use crate::game::viahorizonplus as vhp;
    if screen.id == "multiplayer" {
        vhp::ensure_loaded(|key| game.native_setting(vhp::ID, key));
        vhp::draw_multiplayer_ui(game, batch, screen);
    }
}
