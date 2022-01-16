use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        visible_alias = "i",
        about = "Install a RimWorld Mod by name or ID"
    )]
    Install {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    },

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        visible_alias = "s",
        about = "Search for mods by id or name in Steam within your terminal"
    )]
    Search {
        /// The name or id of the RimWorld mod
        #[clap(required = true)]
        r#mod: String,
    },

    #[clap(
        visible_alias = "l",
        about = crate::statics::LIST_DESCRIPTION)
    ]
    List {
        /// The path where RimWorld is installed
        #[clap(required = false, default_value = "None", env="GAME_PATH")]
        game_path: String,

        /// The path where RimWorld is installed
        #[clap(short, long)]
        large: bool,
    },
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
