extern crate core;

use crate::args::Options;
use std::collections::HashSet;
use rrm_installer::get_or_create_config_dir;

mod args;
mod async_installer;
mod install;
mod list;
mod logger;
mod pull;
mod search;
mod utils;

#[tokio::main]
async fn main() {
    let args: args::App = args::App::load();

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

        args::Commands::Pull { args, ignored } => {
            pull::pull(args, installer, ignored).await;
        }

        args::Commands::List { display } => list::list(installer, display),

        args::Commands::Search { command } => match command {
            args::Search::Local { args } => {
                search::search_locally(installer, args);
            }
            args::Search::Steam { args } => {
                search::search_steam(installer, args).await;
            }
        },

        args::Commands::SearchLocally { args } => {
            search::search_locally(installer, args);
        }

        args::Commands::SearchSteam { args } => {
            search::search_steam(installer, args).await;
        }

        args::Commands::Install { args } => {
            install::install(args, installer, 0, HashSet::new()).await;
        }
    };
}
