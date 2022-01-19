use std::process::exit;
use rwm_locals::GamePath;
use rwm_installer::Installer;

mod args;
mod list;
mod search;
mod utils;

macro_rules! d {
    ($value: expr) => {
        rwm_locals::DisplayType::from($value)
    };
}

#[tokio::main]
async fn main() {
    let args = args::App::load();
    let path = args.game_path.unwrap_or_else(
        || {
            let installer = Installer::new(Some(GamePath::from(&utils::try_get_path(None).rim_install.unwrap_or_else(
                || {
                    eprintln!("Error");
                    exit(1);
                }
            ))));
            if let Some(installer) = installer {
                installer.rim_install.unwrap_or_else(|| {
                    eprintln!("Error");
                    exit(1);
                })
            } else {
                eprintln!("Error");
                exit(1);
            }
        }
    );

    match args.command {
        args::Commands::List { large } => list::list(&path, d!(large)),

        args::Commands::Search { command } => match command {
            args::Search::Local {
                 args, large
            } => {
                search::search_locally(&path, &args.string, args.authors, args.version, args.steam_id, args.name, args.all, d!(large));
            }
            args::Search::Steam { r#mod: m, large } => {
                search::search_steam(&m, d!(large)).await;
            }
        },

        args::Commands::SearchLocally {
            args, large
        } => {
            search::search_locally(&path,  &args.string, args.authors, args.version, args.steam_id, args.name, args.all, d!(large));
        }

        args::Commands::SearchSteam { r#mod: m, large } => {
            search::search_steam(&m, d!(large)).await;
        }

        _ => {}
    };
}
