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
            args::Search::Local {
                string: m,
                authors,
                version,
                steam_id,
                name,
                all,
            } => {
                search::search_locally(&path, &m, authors, version, steam_id, name, all);
            }
            args::Search::Steam { r#mod: _m } => {
                todo!()
            }
        },

        args::Commands::SearchLocally {
            string: m,
            authors,
            version,
            steam_id,
            name,
            all,
        } => {
            search::search_locally(&path, &m, authors, version, steam_id, name, all);
        }

        args::Commands::SearchSteam { r#mod: _m } => {
            todo!()
        }

        _ => {}
    };
}
