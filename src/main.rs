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

    let installer = if args.game_path.is_none() {
        utils::try_get_path(None)
    } else {
        utils::try_get_path(Some(&args.game_path.unwrap()))
    };

    match args.command {
        args::Commands::List { large } => list::list(installer, d!(large)),

        args::Commands::Search { command } => match command {
            args::Search::Local { args, large } => {
                search::search_locally(
                    installer,
                    &args.string,
                    args.authors,
                    args.version,
                    args.steam_id,
                    args.name,
                    args.all,
                    d!(large),
                );
            }
            args::Search::Steam { args } => {
                search::search_steam(&args.r#mod, d!(args.large)).await;
            }
        },

        args::Commands::SearchLocally { args } => {
            search::search_locally(
                installer,
                &args.string,
                args.authors,
                args.version,
                args.steam_id,
                args.name,
                args.all,
                d!(args.large),
            );
        }

        args::Commands::SearchSteam { args } => {
            search::search_steam(&args.r#mod, d!(args.large)).await;
        }

        _ => {}
    };
}
