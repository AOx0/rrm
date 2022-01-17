use crate::utils::*;
use std::path::Path;

pub fn list(game_path: &Path, display_type: DisplayType) {
    let mods: GameMods = GameMods::from(try_get_path(game_path)).with_display(display_type);

    mods.display();
}
