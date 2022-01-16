mod statics;
mod args;
mod locals;
mod utils;

use utils::*;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    if let args::Commands::List { game_path, large } = args.command {
        if game_path != "None" {
            if statics::dir_exists(&game_path) {
                list_mods_at(&format!("{}/Mods", game_path), large);
            } else {
                eprintln!("Error: \"{}\" is not a valid RimWorld's installation game path.", game_path)
            }
        } else {
            let mut found = false;
            statics::RW_DEFAULT_PATH.into_iter().for_each(|path| {
                if statics::dir_exists(path) {
                    list_mods_at(&format!("{}/Mods", path), large);
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

pub fn list_mods_at(path: &str, large: bool) {
    let mods = rwm_list::mods_at(path).parse(large);

    if !large { println!("{}", Mod::gen_headers()); }
    mods.iter().for_each(|r#mod| println!("{}", r#mod));
}
