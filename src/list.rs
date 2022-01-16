use std::collections::HashMap;
use std::fs::File;
use rwm_list::*;
use rwm_xml::*;

const L_FIELDS: [&str; 6] = ["version", "identifier", "name", "packageId", "author", "targetVersion"/*, "description"*/];
const S_FIELDS: [&str; 3] = ["version", "name", "author"];


pub fn list_mods_at(path: &str, large: bool) {
    let fields: &[&str] = if large { &L_FIELDS } else { &S_FIELDS };

    let mods = mods_at(path);

    if !large {
        let headers = "".to_string()
            .add_s(&format!("{:>15}", "Steam ID"))
            .add_s(&format!("   {:<10}", "Version"))
            .add_s(&format!("   {:<50}", "Name"))
            .add_s(&format!("   {:<20}", "Author"))
            .add_s(&format!("\n{:>15}", "--------"))
            .add_s(&format!("   {:<10}", "--------"))
            .add_s(&format!("   {:<50}", "--------"))
            .add_s(&format!("   {:<20}", "--------"));

        println!("{}", headers);
    }

    mods.iter().for_each(|m| {
        if large{ println!("Path : {}", m[0].path.parent().unwrap().display()) }
        let values = EVector::build_from(m, &fields);

        if large {
            print_large(values, m)
        } else {
            print_short(values, m)
        }
    })
}

fn print_large(values: EVector, m: &Vec<ModPaths>) {
    let info = values.to_hash();
    let result = "".to_string()
        .add_f(&info,"name", "", |k| format!("Name : {:}", k) )
        .add_f(&info, "version", "",|k| format!(" [v{:}]", k) )
        .add_s(&format!(  "\nSteam ID   : {}", m[0].steam_id))
        .add_f(&info, "packageId",  "",|k| format!("\npackageId  : {}\n", k) )
        .add_f(&info, "identifier", "",|k| format!("identifier : {}\n", k) )
        .add_f(&info, "author", "",|k| format!("by {}\n", k) );

    println!("{}", result);
}

fn print_short(values: EVector, m: &Vec<ModPaths>) {
    let info = values.to_hash();
    let result = "".to_string()
        .add_s(&format!("{:>15}", m[0].steam_id))
        .add_f(&info, "version", " ",|k| format!("   {:<10}", k))
        .add_f(&info, "name", " ",|k| format!("   {:<50}", k))
        .add_f(&info, "author", " ",|k| format!("   {:<20}", k));


    println!("{}", result);
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
    fn format_field(&self, key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String;
}

impl VersionInfo for HashMap<String, String> {
    fn format_field(&self, key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String {
        if self.contains_key(key) {
            msg(&self[key])
        } else {
            if default != "" {
                msg(default)
            } else {
                "".to_string()
            }

        }

    }
}

trait InfoString {
    fn add_f(&self, m: &HashMap<String, String>, key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String;
    fn add_s(&self, msg: &str) -> String;
}

impl InfoString for String {
    fn add_f(&self, m: &HashMap<String, String>,  key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String {
        format!("{self}{}", &m.format_field(key, default, msg ))
    }

    fn add_s(&self, msg: &str) -> String {
        format!("{self}{}", msg)
    }

}