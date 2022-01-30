use crate::utils::*;
use rwm_installer::Installer;

pub fn list(i: Installer, display_type: DisplayType) {
    let mods: GameMods = GameMods::from(i.rim_install.unwrap())
        .with_display(display_type);

    if i.use_more {
        mods.more_display();
    } else {
        mods.display();
    }

}
