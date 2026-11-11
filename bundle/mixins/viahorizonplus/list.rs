// ViaHorizonPlus entry in Options > Mods
fn viahorizonplus_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::viahorizonplus as vhp;
    mods.push(crate::game::native_hooks::NativeMod {
        id: vhp::ID.into(),
        title: format!("ViaHorizonPlus {}", vhp::VERSION),
        description: vhp::DESCRIPTION.into(),
        keys: vec![],
    });
}
