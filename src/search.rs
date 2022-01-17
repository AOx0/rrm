use std::path::Path;
use crate::utils::*;
use fuzzy_matcher::*;

pub fn search_locally(game_path: &Path, word: &str) {
    let mods = GameMods::from(try_get_path(game_path));
    let matcher = skim::SkimMatcherV2::default();

    let matches: Vec<&Mod> = mods
        .iter()
        .filter(|m| matcher.fuzzy_match(&m.name, word).is_some())
        .collect();

    if !matches.is_empty() {
        println!("{}", Mod::gen_headers());

        matches.iter().for_each(|m| m.display(&DisplayType::Short))
    } else {
        println!("No results found")
    }
}
