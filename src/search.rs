use crate::utils::*;
use fuzzy_matcher::*;

pub fn search_locally(
    game_path: &Path,
    word: &str,
    authors: bool,
    version: bool,
    steam_id: bool,
    name: bool,
    all: bool,
    d_type: DisplayType
) {
    let info = try_get_path(Some(game_path));
    let mods = GameMods::from(GamePath::from(&info.rim_install.unwrap()))
        .with_display(d_type);

    let matcher = skim::SkimMatcherV2::default();

    let all_false = !(authors || version || steam_id || name || all);

    let mut size: usize = 0;

    let matches: Vec<&Mod> = mods
        .iter()
        .filter(|m| {


            let result = (if name || all || all_false {
                matcher.fuzzy_match(&m.name, word).is_some()
            } else {
                false
            }) || (if authors || all {
                matcher.fuzzy_match(&m.author, word).is_some()
            } else {
                false
            }) || (if version || all {
                matcher
                    .fuzzy_match(&m.version.clone().unwrap_or_else(|| "".to_string()), word)
                    .is_some()
            } else {
                false
            }) || (if steam_id || all {
                matcher.fuzzy_match(&m.steam_id, word).is_some()
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

pub async fn search_steam(name: &str, d_type: DisplayType) {
    let mods = SteamMods::search(name)
        .await
        .with_display(d_type);

    mods.display();
}