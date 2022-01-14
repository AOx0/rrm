mod statics;
mod args;
mod list;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    if let args::Commands::List { game_path, large } = args.command {
        if game_path != "None" {
            if statics::dir_exists(&game_path) {
                list::list_mods_at(&format!("{}/Mods", game_path), large);
            } else {
                eprintln!("Error: \"{}\" is not a valid RimWorld's installation game path.", game_path)
            }
        } else {
            let mut found = false;
            statics::RW_DEFAULT_PATH.into_iter().for_each(|path| {
                if statics::dir_exists(path) {
                    list::list_mods_at(&format!("{}/Mods", path), large);
                    found = true;
                }
            });

            if !found {
                eprintln!("\
                    Error: Unable to find RimWorld's installation path.\n\
                    Try specifying the path:\n\
                    \trwm list <PATH>        <--- Like this\
                ")
            }
        }
    }
}