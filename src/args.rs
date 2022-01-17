use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,

    /// The path where RimWorld is installed
    #[clap(short, long, env="GAME_PATH", global = true, required = false, default_value = "None")]
    pub(crate) game_path: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(
        visible_alias = "i",
        about = "Install a RimWorld Mod by name or ID"
    )]
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
        setting(AppSettings::Hidden),
        about = "Search for mods locally, where RimWorld is installed",
        override_usage = "rwm search local <MOD>"
    )]
    SearchLocally {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
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
        about = crate::statics::LIST_DESCRIPTION
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
        about = "Search for mods locally, where RimWorld is installed",
    )]
    Local {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    }
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
