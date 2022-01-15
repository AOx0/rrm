use std::collections::HashMap;
use std::fs::File;
use rwm_list::*;
use rwm_xml::*;

const L_FIELDS: [&str; 6] = ["version", "identifier", "name", "packageId", "author", "targetVersion"/*, "description"*/];
const S_FIELDS: [&str; 3] = ["version", "name", "author"];


pub fn list_mods_at(path: &str, large: bool) {
    let fields: &[&str] = if large {
        &L_FIELDS
    } else {
        &S_FIELDS
    };

    let mods = mods_at(path);
    mods.iter().for_each(|m| {
        if large{
            println!("path : {}", m[0].path.to_str().unwrap().to_string().replace("About", ""));
        }

        let mut values = vec![];

        m.iter().for_each(|m| {

            if let Some(about) = &m.about {
                let file = File::open(about.to_str().unwrap()).unwrap();
                let value = file.values_of(&fields);
                value.into_iter().for_each(|value| values.push(value));
            }

            if let Some(manifest) = &m.manifest {
                let file = File::open(manifest.to_str().unwrap()).unwrap();
                let value = file.values_of(&fields);
                value.into_iter().for_each(|value| values.push(value));
            }
        });

        if large {
            values.iter().for_each(|m| {
                println!("{} : {}", m.name, m.value);
            });
        } else {
            print_basic_info(values);
        }

        if large {
            println!("---\n");
        }
    })
}

fn print_basic_info(values: Vec<Element>) {
    let mut basic_info = HashMap::new();
    values.into_iter().for_each(|m| {
        basic_info.insert( m.name, m.value);
    });

    let mut result = String::from("");

    result.push_str(&basic_info.format_field("name", r"VAL"));
    result.push_str(&basic_info.format_field("version", " [vVAL]"));
    result.push_str(&basic_info.format_field("author", "\nby VAL\n"));

    println!("{result}");
}

trait VersionInfo {
    fn format_field(&self, key: &str, msg: &str) -> String;
}

impl VersionInfo for HashMap<String, String> {
    fn format_field(&self, key: &str, msg: &str) -> String {
        if self.contains_key(key) {
            format!("{}", msg.replace("VAL",  &self[key]))
        } else {
            "".to_string()
        }

    }
}
