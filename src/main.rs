extern crate core;

use clap::CommandFactory;
use crate::args::{Commands, Options};
use std::collections::HashSet;

mod args;
mod async_installer;
mod install;
mod list;
mod logger;
mod pull;
mod search;
mod utils;
use clap_complete::{generate, Shell};

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
        args::Commands::GenerateCompletion { shell } => {
            let mut matches = args::App::command();
            match shell.as_str() {
                "bash" => { generate(Shell::Bash, &mut matches, "rrm", &mut std::io::stdout()); }
                "fish" => { generate(Shell::Fish, &mut matches, "rrm", &mut std::io::stdout()); }
                "zsh" => { generate(Shell::Zsh, &mut matches, "rrm", &mut std::io::stdout()); }
                "powershell" => { generate(Shell::PowerShell, &mut matches, "rrm", &mut std::io::stdout()); }
                "elvish" => { generate(Shell::Elvish, &mut matches, "rrm", &mut std::io::stdout()); }
                _ => {
                    eprintln!("Invalid shell");
                    std::process::exit(1);
                }
            }
        },

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
