// Handling clicks in ViaHorizonPlus settings
fn viahorizonplus_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::viahorizonplus as vhp;
    if mod_id != vhp::ID {
        return;
    }
    let mut settings = vhp::settings(|key| game.native_setting(vhp::ID, key));
    settings.click(button);
    vhp::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(vhp::ID, key, &value);
    }
}
