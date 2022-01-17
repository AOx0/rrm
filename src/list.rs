use crate::utils::*;

pub fn list(game_path: &str, display_type: DisplayType) {
    let mods: GameMods = GameMods::from(try_get_path(game_path))
        .with_display(display_type);

    mods.display();
}