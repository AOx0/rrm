use crate::utils::*;
use fuzzy_matcher::*;
use rwm_installer::Installer;
use crate::args::{Local, Steam};

pub fn search_locally(
    i: Installer, args: Local
) {
    let d_type = rwm_locals::DisplayType::from(args.large);
    let mods = GameMods::from(i.rim_install.unwrap())
        .with_display( d_type);

    let matcher = skim::SkimMatcherV2::default();

    let all_false = !(args.authors || args.version || args.steam_id || args.name || args.all);

    let mut size: usize = 0;

    let matches: Vec<&Mod> = mods
        .iter()
        .filter(|m| {
            let result = (if args.name || args.all || all_false {
                matcher.fuzzy_match(&m.name, &args.string).is_some()
            } else {
                false
            }) || (if args.authors || args.all {
                matcher.fuzzy_match(&m.author, &args.string).is_some()
            } else {
                false
            }) || (if args.version || args.all {
                matcher
                    .fuzzy_match(&m.version.clone().unwrap_or_else(|| "".to_string()), &args.string)
                    .is_some()
            } else {
                false
            }) || (if args.steam_id || args.all {
                matcher.fuzzy_match(&m.steam_id, &args.string).is_some()
            } else {
                false
            });

            if result && m.name.len() > size {
                size = m.name.len();
            };

            result
        })
        .collect();

    if !matches.is_empty() {
        if let DisplayType::Short = d_type {
            println!("{}", Mod::gen_headers(size));
        }

        matches.iter().for_each(|m| m.display(&d_type, size))
    } else {
        println!("No results found")
    }
}

pub async fn search_steam(args: Steam) {
    let mods = SteamMods::search(&args.r#mod)
        .await
        .with_display(rwm_locals::DisplayType::from(args.large));

    mods.display();
}
