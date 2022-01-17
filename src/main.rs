mod args;
mod list;
mod search;
mod utils;

#[tokio::main]
async fn main() {
    let args = args::Args::load();
    let path = args.game_path;

    match args.command {
        args::Commands::List { large } => list::list(&path, rwm_locals::DisplayType::from(large)),

        args::Commands::Search { command } => match command {
            args::Search::Local { r#mod: m } => {
                search::search_locally(&path, &m);
            }
            args::Search::Steam { r#mod: m } => {
                todo!()
            }
        },

        args::Commands::SearchLocally { r#mod: m } => {
            search::search_locally(&path, &m);
        }

        args::Commands::SearchSteam { r#mod: m } => {
            todo!()
        }

        _ => {}
    };
}
