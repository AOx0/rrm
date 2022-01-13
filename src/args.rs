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
        about = "List installed Mods in ../Mods/")
    ]
    List {
        /// The path where Mods are installed
        #[clap(required = false, default_value = "None", env="MODS_PATH")]
        mods_path: String,
    },
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
