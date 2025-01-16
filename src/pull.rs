use crate::args::{InstallCommandGroup, InstallingOptions, Pull};
use crate::install::install;
use crate::utils::Path;
use crate::utils::*;
use std::collections::HashSet;

pub async fn pull(args: Pull, i: Installer, ignored: bool) {
    let mods: GameMods =
        GameMods::from(i.rim_install.clone().unwrap()).with_display(DisplayType::Short);

    if args.is_verbose() {
        println!("Listing installed ids: ");
    }

    let ids = mods.iter().filter_map(|mo| {
        if args.is_debug() {
            println!(
                "Turn of {}",
                Path::new(&mo.path).file_name().unwrap().to_str().unwrap()
            );
        }

        if Path::new(&mo.path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .starts_with('_')
            && !ignored
        {
            if args.is_verbose() {
                println!("Ignoring {}", mo.steam_id);
            }
            None
        } else {
            if args.is_verbose() {
                println!("Adding {}", mo.steam_id);
            }
            Some(mo.steam_id.clone())
        }
    });

    let to_install: HashSet<String> = HashSet::from_iter(ids);
    let ids: Vec<String> = to_install.iter().cloned().collect();

    let to_install = InstallCommandGroup {
        rimmod: ids,
        filter: None,
        author: false,
        version: false,
        steam_id: false,
        name: false,
        all: false,
        yes: true,
        resolve: args.resolve,
        verbose: args.verbose,
        debug: args.debug,
    };

    install(to_install, i, 0, HashSet::new()).await;
}
