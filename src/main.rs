
use rwm_locals::DisplayType;

mod statics;
mod args;
mod list;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    match args.command {

        args::Commands::List { game_path, large } => {
            list::list(&game_path, DisplayType::from(large))
        },

        args::Commands::Search { command } => {
            match command {
                args::Search::Local { r#mod: m } => {
                    todo!()
                },
                args::Search::Steam { r#mod: m } => {
                    todo!()
                },
            }
        },

        args::Commands::SearchLocally { r#mod } => {
            todo!()
        },

        args::Commands::SearchSteam { r#mod } => {
            todo!()
        },

        _ => {}
    };
}