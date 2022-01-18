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
    let path = args.game_path;

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
