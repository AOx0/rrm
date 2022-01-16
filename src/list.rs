use std::collections::HashMap;
use std::fs::File;
use rwm_list::*;
use rwm_xml::*;

const L_FIELDS: [&str; 6] = ["version", "identifier", "name", "packageId", "author", "targetVersion"/*, "description"*/];
const S_FIELDS: [&str; 3] = ["version", "name", "author"];


pub fn list_mods_at(path: &str, large: bool) {
    let fields: &[&str] = if large { &L_FIELDS } else { &S_FIELDS };

    let mods = mods_at(path);
    mods.iter().for_each(|m| {
        if large{ println!("Path : {}", m[0].path.parent().unwrap().display()) }
        let values = EVector::build_from(m, &fields);
        if large { print_large(values, m) } else { print_short(values, m) }
    })
}

fn print_large(values: EVector, m: &Vec<ModPaths>) {
    let info = values.to_hash();
    let mut result = String::from("");

    result.push_str(&info.format_field("name", "Name : VAL"));
    result.push_str(&info.format_field("version", " [vVAL]"));
    result.push_str(&info.format_field("packageId",  "\npackageId  : VAL\n"));
    result.push_str(&info.format_field("identifier", "identifier : VAL\n"));
    result.push_str(&info.format_field("author", "by VAL"));
    result.push_str(&format!(" [steamID: {}]\n", m[0].steam_id));

    println!("{result}");
}

fn print_short(values: EVector, m: &Vec<ModPaths>) {
    let info = values.to_hash();
    let mut result = String::from("");

    result.push_str(&info.format_field("name", r"VAL"));
    result.push_str(&info.format_field("version", " [vVAL]"));
    result.push_str(&info.format_field("author", "\nby VAL"));
    result.push_str(&format!(" [steamID: {}]\n", m[0].steam_id));


    println!("{result}");
}

type EVector =  Vec<Element>;

trait ElementVector {
    fn to_hash(self) -> HashMap<String, String>;
    fn build_from(m: &Vec<ModPaths>, with_fields: &[&str]) -> EVector;
}

impl ElementVector for EVector {
    fn to_hash(self) -> HashMap<String, String> {
        let mut basic_info = HashMap::new();
        self.into_iter().for_each(|m| {
            basic_info.insert( m.name, m.value);
        });

        basic_info
    }

    fn build_from(m: &Vec<ModPaths>, with_fields: &[&str]) -> EVector {
        let mut values = vec![];
        m.iter().for_each(|m| {

            if let Some(about) = &m.about {
                let file = File::open(about.to_str().unwrap()).unwrap();
                let value = file.values_of(&with_fields);
                value.into_iter().for_each(|value| values.push(value));
            }

            if let Some(manifest) = &m.manifest {
                let file = File::open(manifest.to_str().unwrap()).unwrap();
                let value = file.values_of(&with_fields);
                value.into_iter().for_each(|value| values.push(value));
            }
        });
        values
    }
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
