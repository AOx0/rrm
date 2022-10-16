use rrm_xml::{Element, XMLFile};
use std::collections::HashMap;

use crate::Mods;
use path_absolutize::Absolutize;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ModPaths {
    pub about: Option<PathBuf>,
    pub manifest: Option<PathBuf>,
    pub path: PathBuf,
    pub steam_id: String,
}

fn list_p(path: &Path) -> Vec<PathBuf> {
    let contents = path.absolutize().unwrap().read_dir().unwrap();
    let mut result = vec![];
    for e in contents {
        result.push(e.unwrap().path());
    }
    result
}

fn list_b(buf: &Path) -> Vec<PathBuf> {
    list_p(buf)
}

fn get_mods(about_dir: &Path) -> Vec<ModPaths> {
    let mut mod_files = vec![];
    list_b(about_dir).iter().for_each(|path| {
        let parent = path.parent().unwrap();
        let steam_id = parent.join("PublishedFileId.txt");

        let file: std::io::Result<File> = File::open(steam_id);
        let mut steam_id: Vec<u8> = Vec::new();

        if file.is_err() {
            steam_id = Vec::from("NOT FOUND".as_bytes());
            eprintln!(
                "Warning: Could not find PublishedFileId.txt in path {}",
                parent.display()
            );
        } else {
            file.unwrap().read_to_end(&mut steam_id).unwrap();
        }

        let m = ModPaths {
            about: if path.file_name().unwrap().to_str().unwrap().to_lowercase() == "about.xml" {
                Some(PathBuf::from(path))
            } else {
                None
            },
            manifest: if path.file_name().unwrap().to_str().unwrap().to_lowercase()
                == "manifest.xml"
            {
                Some(PathBuf::from(path))
            } else {
                None
            },
            path: PathBuf::from(parent),
            steam_id: String::from_utf8(steam_id)
                .unwrap()
                .replace("\n", "")
                .replace(" ", ""),
        };

        if m.about.is_some() || m.manifest.is_some() {
            mod_files.push(m);
        }
    });

    mod_files
}

pub fn mods_at(path: &Path) -> Vec<Vec<ModPaths>> {
    let mut r: Vec<Vec<ModPaths>> = vec![];
    list_path_abouts(path)
        .into_iter()
        .for_each(|about| r.push(get_mods(&about)));
    r
}

pub fn list_path_abouts(path: &Path) -> Vec<PathBuf> {
    let mut result = vec![];

    for e in list_p(path) {
        if e.is_dir() {
            list_b(&e).iter().for_each(|e| {
                if e.file_name().unwrap().to_str().unwrap().to_lowercase() == "about" {
                    result.push(PathBuf::from(e));
                }
            });
        }
    }

    result
}

pub trait ModVec {
    fn parse(self) -> (Mods, usize);
    fn load_from_path(path: &Path) -> (Mods, usize);
}

impl ModVec for Vec<Vec<ModPaths>> {
    fn parse(self) -> (Mods, usize) {
        const L_FIELDS: [&str; 6] = [
            "version",
            "identifier",
            "name",
            "packageId",
            "author",
            "targetVersion",
        ];

        let mut biggest_name_size: usize = 0;
        let mut mods = vec![];
        self.iter().for_each(|m| {
            if m.is_empty() {
                return;
            }

            let values = EVector::build_from(m, &L_FIELDS);
            let m = values.clone().to_mod(m.get(0).unwrap_or_else(|| {
                println!("Error while getting index 0 of {:?}", m);
                println!("E_vec: {:?}", values);
                println!("Self: {:?}", self);
                panic!();
            }));
            let name_size = m.name.len();
            if name_size > biggest_name_size {
                biggest_name_size = name_size;
            }
            mods.push(m);
        });

        (mods, biggest_name_size)
    }

    fn load_from_path(path: &Path) -> (Mods, usize) {
        crate::mods_at(path).parse()
    }
}

pub type EVector = Vec<Element>;

pub trait ElementVector {
    fn to_hash(self) -> (HashMap<String, String>, Option<Vec<String>>);
    fn to_mod(self, m: &ModPaths) -> crate::mod_obj::Mod;
    fn build_from(m: &[ModPaths], with_fields: &[&str]) -> EVector;
}

impl ElementVector for EVector {
    fn to_hash(self) -> (HashMap<String, String>, Option<Vec<String>>) {
        let mut basic_info = HashMap::new();
        let mut dependencies: Vec<String> = Vec::new();
        self.into_iter().for_each(|m| {
            if m.name == "steamWorkshopUrl" {
                dependencies.push(m.value)
            } else {
                basic_info.insert(m.name, m.value);
            }
        });

        let dependencies = if dependencies.is_empty() {
            None
        } else {
            Some(dependencies)
        };
        (basic_info, dependencies)
    }

    fn to_mod(self, m: &ModPaths) -> crate::mod_obj::Mod {
        crate::mod_obj::Mod::from_evec(self, m)
    }

    fn build_from(m: &[ModPaths], with_fields: &[&str]) -> EVector {
        let mut values = vec![];
        m.iter().for_each(|m| {
            if let Some(about) = &m.about {
                let file = File::open(about.to_str().unwrap()).unwrap();
                let value = file.values_of(with_fields);
                value.into_iter().for_each(|value| values.push(value));
            }

            if let Some(manifest) = &m.manifest {
                let file = File::open(manifest.to_str().unwrap()).unwrap();
                let value = file.values_of(with_fields);
                value.into_iter().for_each(|value| values.push(value));
            }
        });
        values
    }
}
