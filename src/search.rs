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
) {
    let mods = GameMods::from(try_get_path(game_path));
    let matcher = skim::SkimMatcherV2::default();

    let all_false = !(authors || version || steam_id || name || all);

    let matches: Vec<&Mod> = mods
        .iter()
        .filter(|m| {
            (if name || all || all_false {
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
            })
        })
        .collect();

    if !matches.is_empty() {
        println!("{}", Mod::gen_headers());

        matches.iter().for_each(|m| m.display(&DisplayType::Short))
    } else {
        println!("No results found")
    }
}
