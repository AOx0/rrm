mod args;
mod list;
mod search;
mod utils;

#[tokio::main]
async fn main() {
    let args = args::App::load();
    let path = args.game_path;

    match args.command {
        args::Commands::List { large } => list::list(&path, rwm_locals::DisplayType::from(large)),

        args::Commands::Search { command } => match command {
            args::Search::Local {
                 args,
            } => {
                search::search_locally(&path, &args.string, args.authors, args.version, args.steam_id, args.name, args.all);
            }
            args::Search::Steam { r#mod: _m } => {
                todo!()
            }
        },

        args::Commands::SearchLocally {
            args
        } => {
            search::search_locally(&path, &args.string, args.authors, args.version, args.steam_id, args.name, args.all);
        }

        args::Commands::SearchSteam { r#mod: _m } => {
            todo!()
        }

        _ => {}
    };
}
