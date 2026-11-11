// Mouse wheel hook for scrolling through the version dropdown
fn viahorizonplus_screen_scroll(
    game: &crate::game::Game<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    let _ = game;
    use crate::game::viahorizonplus as vhp;
    if vhp::on_multiplayer_scroll(scroll[1]) {
        *canceled = true;
    }
}
