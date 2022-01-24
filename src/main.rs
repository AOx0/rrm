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
            args::Search::Local { args } => {
                let f = args.to_filter_obj();
                println!("{:?}", f);
                search::search_locally(installer, args);
            }
            args::Search::Steam { args } => {
                search::search_steam(args).await;
            }
        },

        args::Commands::SearchLocally { args } => {
            search::search_locally(installer, args);
        }

        args::Commands::SearchSteam { args } => {
            search::search_steam(args).await;
        }

        _ => {}
    };
}
