use crate::args::{Install, Pull};
use crate::install::install;
use crate::utils::Path;
use crate::utils::*;
use std::borrow::Borrow;
use std::collections::HashSet;

pub async fn pull(args: Pull, i: Installer, ignored: bool) {
    let mods: GameMods =
        GameMods::from(i.rim_install.clone().unwrap()).with_display(DisplayType::Short);

    let ids = mods
        .iter()
        .map(|mo| {
            if Path::new(&mo.path)
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()[0usize..1usize]
                .borrow()
                != "_"
                && !ignored
            {
                Some(mo.steam_id.clone())
            } else {
                None
            }
        })
        .flatten();

    let to_install: HashSet<String> = HashSet::from_iter(ids);
    let ids: Vec<String> = to_install.iter().cloned().collect();

    let to_install = Install {
        r#mod: ids,
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
