use crate::utils::*;

pub fn list(game_path: &Path, display_type: DisplayType) {
    let installer = try_get_path(Some(game_path));
    let mods: GameMods = GameMods::from(GamePath::from(&installer.rim_install.unwrap())).with_display(display_type);

    mods.display();
}
