use crate::args::Options;

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

    let mut installer = utils::try_get_path(
        None,
        matches!(
            &args.command,
            args::Commands::Set {
                command: Options::GamePath { .. }
            }
        ),
    );

    match args.command {
        args::Commands::Set { command } => match command {
            Options::UsePager { value } => {
                installer.set_more_value(value == "true" || value == "1")
            }

            Options::GamePath { value } => {
                installer.set_path_value(value);
            }

            Options::Pager { value } => {
                installer.set_paging_software(value.to_str().unwrap());
            }
        },

        args::Commands::List { large } => list::list(installer, d!(large)),

        args::Commands::Search { command } => match command {
            args::Search::Local { args } => {
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
