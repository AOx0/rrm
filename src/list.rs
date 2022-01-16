use std::process::exit;
use rwm_locals::{DisplayType, GameMods};
use crate::statics;

pub fn list(game_path: &str, display_type: DisplayType) {
    if game_path != "None" {
        if statics::dir_exists(game_path) {
            list_mods_at(game_path, display_type);
        } else {
            eprintln!("Error: \"{}\" is not a valid RimWorld installation path.", game_path)
        }
    } else {
        let mut found = false;
        statics::RW_DEFAULT_PATH.into_iter().for_each(|path| {
            if statics::dir_exists(path) {
                list_mods_at(path, display_type);
                found = true;
            }
        });

        if !found {
            eprintln!("\
                Error: Unable to find RimWorld installation path.\n\
                Try specifying the path:\n\
                \trwm list <PATH>        <--- Like this\
            ");
            exit(1);
        }
    }
}

fn list_mods_at(path: &str, display_type: DisplayType) {
    let mods: GameMods = GameMods::from(path)
        .with_display(display_type);

    mods.display();
}
