use rwm_list::*;
use crate::locals::Mod;
use crate::utils::{ElementVector, EVector};

const L_FIELDS: [&str; 6] = ["version", "identifier", "name", "packageId", "author", "targetVersion"/*, "description"*/];
const S_FIELDS: [&str; 3] = ["version", "name", "author"];


pub fn list_mods_at(path: &str, large: bool) {
    let fields: &[&str] = if large { &L_FIELDS } else { &S_FIELDS };

    let mods = mods_at(path);

    if !large { println!("{}", Mod::gen_headers()); }

    mods.iter().for_each(|m| {
        let values = EVector::build_from(m, &fields);
        let values = values.to_mod(&m.get(0).unwrap(), large);

        println!("{}", values);
    })
}

