use crate::mod_paths::ModPaths;
use crate::*;
use std::io::{Stdout, Write};

#[derive(Clone, Copy)]
pub enum DisplayType {
    Short,
    Long,
}

impl From<bool> for DisplayType {
    fn from(t: bool) -> Self {
        if t {
            DisplayType::Long
        } else {
            DisplayType::Short
        }
    }
}

#[derive(Clone)]
pub struct Mod {
    pub path: String,
    pub name: String,
    pub author: String,
    pub steam_id: String,
    pub version: Option<String>,
    pub package_id: Option<String>,
    pub identifier: Option<String>,
}

impl Mod {
    pub fn from_evec(e_vec: EVector, m: &ModPaths) -> Self {
        let mods = e_vec.to_hash();

        Mod {
            path: m.path.parent().unwrap().display().to_string(),
            name: mods["name"].clone(),
            author: mods["author"].clone(),
            steam_id: m.steam_id.clone(),
            version: mods.get("version").cloned(),
            package_id: mods.get("packageId").cloned(),
            identifier: mods.get("identifier").cloned(),
        }
    }

    pub fn gen_headers() -> String {
        "".to_string()
            .add_s(format!("{:>15}", "Steam ID"))
            .add_s(format!("   {:<10}", "Version"))
            .add_s(format!("   {:<50}", "Name"))
            .add_s(format!("   {:<20}", "Author"))
            .add_s(format!("\n{:>15}", "--------"))
            .add_s(format!("   {:<10}", "--------"))
            .add_s(format!("   {:<50}", "--------"))
            .add_s(format!("   {:<20}", "--------"))
    }

    pub fn gen_large(&self) -> String {
        let mut result = ""
            .to_string()
            .add_s(format!("Path : {:}\n", self.path))
            .add_s(format!("Name : {:}", self.name));

        if self.version.is_some() {
            result.push_str(&format!(" [v{:}]", self.version.clone().unwrap()))
        }

        result.push_str(&format!("\nSteam ID   : {}", self.steam_id));

        if self.package_id.is_some() {
            result.push_str(&format!(
                "\npackageId  : {}\n",
                self.package_id.clone().unwrap()
            ))
        }

        if self.identifier.is_some() {
            result.push_str(&format!(
                "identifier : {}\n",
                self.identifier.clone().unwrap()
            ))
        }

        result.push_str(&format!("by {}\n", self.author));

        result
    }

    pub fn gen_short(&self) -> String {
        "".to_string()
            .add_s(format!("{:>15}", self.steam_id))
            .add_s(format!(
                "   {:<10}",
                self.version.clone().unwrap_or(" ".to_string())
            ))
            .add_s(format!("   {:<50}", self.name))
            .add_s(format!("   {:<20}", self.author))
    }

    pub fn display(&self, form: &DisplayType) {
        let mut f: Stdout = std::io::stdout();

        if let DisplayType::Long = form {
            write!(f, "{}\n", self.gen_large()).unwrap()
        } else {
            write!(f, "{}\n", self.gen_short()).unwrap()
        }
    }
}

trait InfoString {
    fn add_s(&self, msg: String) -> String;
}

impl InfoString for String {
    fn add_s(&self, msg: String) -> String {
        format!("{self}{}", msg)
    }
}
