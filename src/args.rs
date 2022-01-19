use crate::utils::*;
use clap::{AppSettings, Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
pub struct App {
    #[clap(subcommand)]
    pub(crate) command: Commands,

    /// The path where RimWorld is installed
    #[clap(
        short,
        long,
        env = "GAME_PATH",
        global = true,
        required = false,
    )]
    pub(crate) game_path: Option<PathBuf>,
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
        /// The name of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,

        /// Display the larger message
        #[clap(short, long)]
        large: bool,
    },

    #[clap(
        visible_alias = "sl",
        setting(AppSettings::Hidden | AppSettings::DisableVersionFlag),
        about = "Search for mods locally, where RimWorld is installed [with no flags searches by name]",
        override_usage = "rwm search local [OPTIONS] <STRING>"
    )]
    SearchLocally {
        #[clap(flatten)]
        args: Local,

        /// Display the larger message
        #[clap(short, long)]
        large: bool,
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
        /// Display the larger message
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
        /// The name of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,

        /// Display the larger message
        #[clap(short, long)]
        large: bool,
    },

    #[clap(
        visible_aliases = &["l", "sl (global)"],
        setting(AppSettings::DisableVersionFlag),
        about = "Search for mods locally, where RimWorld is installed [with no flags searches by name]",
    )]
    Local {
        #[clap(flatten)]
        args: Local,

        /// Display the larger message
        #[clap(short, long)]
        large: bool,
    },
}

#[derive(Args, Debug)]
pub struct Local {
    /// The pattern to search
    #[clap(required = true)]
    pub(crate) r#string: String,

    /// Search by author(s) name(s)
    #[clap(short, long)]
    pub(crate) authors: bool,

    /// Search by version
    #[clap(short, long)]
    pub(crate) version: bool,

    /// Search by Steam ID
    #[clap(short, long)]
    pub(crate) steam_id: bool,

    /// Search by mod name
    #[clap(short, long)]
    pub(crate) name: bool,

    /// Search by all fields
    #[clap(long, conflicts_with_all = &["authors", "version", "steam-id", "name"])]
    pub(crate) all: bool,
}


impl App {
    pub fn load() -> App {
        App::parse()
    }
}

