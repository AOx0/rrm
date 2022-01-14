mod statics;
mod args;
mod list;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    if let args::Commands::List {  mods_path } = args.command {
        if mods_path == "None" {
            for path in statics::RW_DEFAULT_PATH {
                if statics::dir_exists(path) {
                    list::list_mods_at(&format!("{}/Mods", statics::RW_DEFAULT_PATH[0]));
                }
            }
        } else {
            if statics::dir_exists(&mods_path) {
                list::list_mods_at(&format!("{}/Mods", mods_path));
            }
        }
    }
}