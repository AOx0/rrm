use std::fs::File;
use rwm_list::*;
use rwm_xml::*;

const FIELDS: [&str; 6] = ["version", "identifier", "name", "packageId", "author", "targetVersion"/*, "description"*/];

pub fn list_mods_at(path: &str) {
    let mods = mods_at(path);
    mods.iter().for_each(|m| {
        println!("path : {}", m[0].path.to_str().unwrap().to_string().replace("About", ""));
        m.iter().for_each(|m| {

            let mut values = vec![];
            if let Some(about) = &m.about {
                let file = File::open(about.to_str().unwrap()).unwrap();
                let value = file.values_of(&FIELDS);
                value.into_iter().for_each(|value| values.push(value));
            }

            if let Some(manifest) = &m.manifest {
                let file = File::open(manifest.to_str().unwrap()).unwrap();
                let value = file.values_of(&FIELDS);
                value.into_iter().for_each(|value| values.push(value));
            }

            values.iter().for_each(|m| {
                println!("{} : {}", m.name, m.value);
            });

        });

        println!("---\n");
    })
}