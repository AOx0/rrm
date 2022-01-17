use crate::utils::*;
use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,

    /// The path where RimWorld is installed
    #[clap(
        short,
        long,
        env = "GAME_PATH",
        global = true,
        required = false,
        default_value = "None"
    )]
    pub(crate) game_path: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(visible_alias = "i", about = "Install a RimWorld Mod by name or ID")]
    Install {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    },

    #[clap(
        visible_alias = "ss",
        setting(AppSettings::Hidden),
        about = "Search for mods in Steam",
        override_usage = "rwm search steam <MOD>"
    )]
    SearchSteam {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    },

    #[clap(
        visible_alias = "sl",
        setting(AppSettings::Hidden | AppSettings::DisableVersionFlag),
        about = "Search for mods locally, where RimWorld is installed [with no flags searches by name]",
        override_usage = "rwm search local [OPTIONS] <STRING>"
    )]
    SearchLocally {
        /// The pattern to search
        #[clap(required = true)]
        r#string: String,

        /// Search by author(s) name(s)
        #[clap(short, long)]
        authors: bool,

        /// Search by version
        #[clap(short, long)]
        version: bool,

        /// Search by Steam ID
        #[clap(short, long)]
        steam_id: bool,

        /// Search by mod name
        #[clap(short, long)]
        name: bool,

        /// Search by all fields
        #[clap(long, conflicts_with_all = &["authors", "version", "steam-id", "name"])]
        all: bool,
    },

    #[clap(
        visible_alias = "s",
        about = "Search for mods by id or name locally or in Steam within your terminal"
    )]
    Search {
        #[clap(subcommand)]
        command: Search,
    },

    #[clap(
        visible_alias = "l",
        about = LIST_DESCRIPTION
    )]
    List {
        /// The path where RimWorld is installed
        #[clap(short, long)]
        large: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum Search {
    #[clap(
        visible_aliases = &["s", "ss (global)"],
        about = "Search for mods in Steam",
    )]
    Steam {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    },

    #[clap(
        visible_aliases = &["l", "sl (global)"],
        setting(AppSettings::DisableVersionFlag),
        about = "Search for mods locally, where RimWorld is installed [with no flags searches by name]",
    )]
    Local {
        /// The pattern to search
        #[clap(required = true)]
        r#string: String,

        /// Search by author(s) name(s)
        #[clap(short, long)]
        authors: bool,

        /// Search by version
        #[clap(short, long)]
        version: bool,

        /// Search by Steam ID
        #[clap(short, long)]
        steam_id: bool,

        /// Search by mod name
        #[clap(short, long)]
        name: bool,

        /// Search by all fields
        #[clap(long, conflicts_with_all = &["authors", "version", "steam-id", "name"])]
        all: bool,
    },
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
