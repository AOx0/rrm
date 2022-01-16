use rwm_locals::{DisplayType, GameMods};

mod statics;
mod args;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    if let args::Commands::List { game_path, large } = args.command {
        if game_path != "None" {
            if statics::dir_exists(&game_path) {
                list_mods_at(&game_path, DisplayType::from(large));
            } else {
                eprintln!("Error: \"{}\" is not a valid RimWorld installation path.", game_path)
            }
        } else {
            let mut found = false;
            statics::RW_DEFAULT_PATH.into_iter().for_each(|path| {
                if statics::dir_exists(path) {
                    list_mods_at(path, DisplayType::from(large));
                    found = true;
                }
            });

            if !found {
                eprintln!("\
                    Error: Unable to find RimWorld installation path.\n\
                    Try specifying the path:\n\
                    \trwm list <PATH>        <--- Like this\
                ")
            }
        }
    }
}

pub fn list_mods_at(path: &str, display_type: DisplayType) {
    let mods: GameMods = GameMods::from(path)
        .with_display(display_type);

    mods.display();
}
