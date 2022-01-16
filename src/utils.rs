use std::collections::HashMap;
use std::fs::File;
use rwm_xml::{Element, XMLFile};
use rwm_list::ModPaths;

pub trait InfoString {
    fn add_f(&self, m: &HashMap<String, String>, key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String;
    fn add_s(&self, msg: String) -> String;
}

impl InfoString for String {
    fn add_f(&self, m: &HashMap<String, String>,  key: &str, default: &str, msg: impl FnOnce(&str) -> String) -> String {
        format!("{self}{}", &m.format_field(key, default, msg ))
    }

    fn add_s(&self, msg: String) -> String {
        format!("{self}{}", msg)
    }

}

pub type EVector =  Vec<Element>;

pub trait ElementVector {
    fn to_hash(self) -> HashMap<String, String>;
    fn to_mod(self, m: &ModPaths, large: bool) -> crate::locals::Mod;
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

    fn to_mod(self, m: &ModPaths, large: bool) -> crate::locals::Mod {
        crate::locals::Mod::from_evec(self, &m, large)
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
