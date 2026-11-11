// Mouse click hook for ViaHorizonPlus UI on the Multiplayer screen
fn viahorizonplus_screen_mouse(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    use crate::game::viahorizonplus as vhp;
    if screen.id == "multiplayer" && button == 0 {
        let (w, h) = (screen.rect[2], screen.rect[3]);
        if vhp::on_multiplayer_click(screen.mouse, w, h) {
            *canceled = true;
            let settings = vhp::get_settings();
            for (key, value) in settings.stored() {
                game.native_set_setting(vhp::ID, key, &value);
            }
        }
    }
}
