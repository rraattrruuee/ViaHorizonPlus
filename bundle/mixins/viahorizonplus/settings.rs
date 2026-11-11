// ViaHorizonPlus settings page in Options > Mods
fn viahorizonplus_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::viahorizonplus as vhp;
    if mod_id != vhp::ID {
        return;
    }
    let game = &*game;
    let settings = vhp::settings(|key| game.native_setting(vhp::ID, key));
    buttons.extend(settings.labels());
}
