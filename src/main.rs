extern crate core;

use crate::args::Options;
use clap::CommandFactory;
use std::{collections::HashSet, process::ExitCode};

mod args;
mod async_installer;
mod install;
mod list;
mod logger;
mod pull;
mod search;
mod utils;
use clap_complete::{generate, Shell};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> ExitCode {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    if app().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[tokio::main]
async fn app() -> Result<(), ExitCode> {
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
        args::Commands::Completions { shell } => {
            let mut matches = args::App::command();
            let shell = match shell.as_str() {
                "bash" => Shell::Bash,
                "fish" => Shell::Fish,
                "zsh" => Shell::Zsh,
                "powershell" => Shell::PowerShell,
                "elvish" => Shell::Elvish,
                _ => {
                    eprintln!("Invalid shell");
                    return Err(ExitCode::FAILURE);
                }
            };

            generate(shell, &mut matches, "rrm", &mut std::io::stdout());
            Ok(())
        }

        args::Commands::Set { command } => match command {
            Options::UsePager { value } => {
                installer.set_more_value(value == "true" || value == "1");
                Ok(())
            }

            Options::GamePath { value } => {
                installer.set_path_value(value);
                Ok(())
            }

            Options::Pager { value } => {
                installer.set_paging_software(value.to_str().unwrap());
                Ok(())
            }
        },

        args::Commands::Pull { args, ignored } => {
            pull::pull(args, installer, ignored).await;
            Ok(())
        }

        args::Commands::List { display } => {
            list::list(installer, display);
            Ok(())
        }

        args::Commands::Search { command } => match command {
            args::Search::Local { args } => {
                search::search_locally(installer, args);
                Ok(())
            }
            args::Search::Steam { args } => {
                search::search_steam(installer, args).await;
                Ok(())
            }
        },

        args::Commands::SearchLocally { args } => {
            search::search_locally(installer, args);
            Ok(())
        }

        args::Commands::SearchSteam { args } => {
            search::search_steam(installer, args).await;
            Ok(())
        }

        args::Commands::Install { args } => {
            install::install(args, installer, 0, HashSet::new()).await;
            Ok(())
        }
    }
}
