use crate::utils::*;
use clap::{AppSettings, Args, Parser, Subcommand};
use rwm_locals::{FilterBy, FlagSet};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
#[clap(override_help = "\
rwm-set
Set new configuration values

USAGE:
    rwm set <OPTION> <VALUE>

OPTIONS:
    game-path    Set the path where RimWorld is installed [alias: 'path']
    pager        Set the paging software to use, like bat, more or less [alias: 'paging']
    use-pager    Set if rwm should use more to display output [values: false, true, 0, 1] [alias: 'use-paging']
")]
pub enum Options {
    #[clap(
        about = "Set if rwm should use paging software to display output [values: false, true, 0, 1]",
        visible_alias = "use-paging"
    )]
    UsePager {
        #[clap(required = true, possible_values= &["true", "false", "0", "1"])]
        value: String,
    },

    #[clap(
        about = "Set the path where RimWorld is installed",
        visible_alias = "path"
    )]
    GamePath {
        /// The path where RimWorld is installed
        #[clap(required = true)]
        value: PathBuf,
    },

    #[clap(
        about = "Set the paging software to use, like bat, more or less",
        visible_alias = "paging"
    )]
    Pager {
        #[cfg(target_os = "windows")]
        /// The path where the paging software is, for example: C:\Windows\System32\more.com
        #[clap(required = true)]
        value: PathBuf,

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        /// The name of the paging software, for example: bat, more
        #[clap(required = true)]
        value: PathBuf,
    },
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
        #[clap(flatten)]
        args: Steam,
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
    },

    #[clap(about = "Set new configuration values")]
    Set {
        #[clap(subcommand)]
        command: Options,
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
        #[clap(flatten)]
        args: Steam,
    },

    #[clap(
        visible_aliases = &["l", "sl (global)"],
        setting(AppSettings::DisableVersionFlag),
        about = "Search for mods locally, where RimWorld is installed [with no flags searches by name]",
    )]
    Local {
        #[clap(flatten)]
        args: Local,
    },
}

#[derive(Args, Debug)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
pub struct Steam {
    /// The name of the RimWorld mod
    #[clap(required = true)]
    pub(crate) r#mod: String,

    /// Display the larger message
    #[clap(short, long)]
    pub(crate) large: bool,
}

#[derive(Args, Debug)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
pub struct Local {
    /// The pattern to search
    #[clap(required = true)]
    pub(crate) r#string: String,

    /// Search by author(s) name(s)
    #[clap(short, long)]
    pub(crate) author: bool,

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

    /// Display the larger message
    #[clap(short, long)]
    pub(crate) large: bool,
}

macro_rules! a_if {
    ($cond: expr, $add: expr) => {
        if $cond {
            $add
        } else {
            FilterBy::None
        }
    };
}

impl Local {
    pub fn to_filter_obj(&self) -> FlagSet<FilterBy> {
        let mut result: FlagSet<FilterBy> = FlagSet::from(FilterBy::None);

        if self.all {
            return FlagSet::from(FilterBy::All);
        }

        result |= a_if!(self.name, FilterBy::Name);
        result |= a_if!(self.author, FilterBy::Author);
        result |= a_if!(self.version, FilterBy::Version);
        result |= a_if!(self.steam_id, FilterBy::SteamID);

        result -= FilterBy::None;

        if result.is_empty() {
            result |= FilterBy::Name;
        }

        result
    }
}

impl App {
    pub fn load() -> App {
        App::parse()
    }
}
