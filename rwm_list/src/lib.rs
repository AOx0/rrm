use std::fs::File;
use std::io::Read;
pub use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};

fn list_p(path: &Path) -> Vec<PathBuf> {
    let contents = path.absolutize().unwrap().read_dir().unwrap();
    let mut result = vec![];
    for e  in contents {
        result.push( e.unwrap().path());
    }
    result
}

fn list_b(buf: &PathBuf) -> Vec<PathBuf> {
    list_p(buf.as_path())
}

pub fn list_path_abouts(path: &str)  -> Vec<PathBuf> {
    let mut result = vec![];
    let path = Path::new(path);
    for e  in list_p(path) {
        if e.is_dir() {
            list_b(&e).iter().for_each(|e|
                if e.file_name().unwrap() == "About" {
                    result.push(PathBuf::from(e));
                }
            );
        }
    }

    result
}

#[derive(Debug)]
pub struct ModPaths {
    pub about: Option<PathBuf>,
    pub manifest: Option<PathBuf>,
    pub path: PathBuf,
    pub steam_id: String
}

fn get_mods(about_dir: &PathBuf) -> Vec<ModPaths> {
    let mut mod_files = vec![];
    list_b(about_dir)
        .iter()
        .for_each(|path| {
            let parent = path.parent().unwrap();
            let steam_id = parent.join("PublishedFileId.txt");

            let mut file = File::open(steam_id.clone()).unwrap();
            let mut steam_id: Vec<u8> = Vec::new();
            file.read_to_end(&mut steam_id).unwrap();

            let m = ModPaths {
                about: if path.file_name().unwrap() == "About.xml" { Some(PathBuf::from(path)) } else { None },
                manifest: if path.file_name().unwrap() == "Manifest.xml" { Some(PathBuf::from(path)) } else { None },
                path: PathBuf::from(parent),
                steam_id: String::from_utf8(steam_id).unwrap().replace("\n", "").replace(" ", "")
            };

            if m.about.is_some() || m.manifest.is_some() {
                mod_files.push(m);
            }

        }
        );

    mod_files
}

pub fn mods_at(path: &str) -> Vec<Vec<ModPaths>> {
    let mut r: Vec<Vec<ModPaths>> = vec![];
    list_path_abouts(path).into_iter().for_each(
        |about| {
            r.push(get_mods(&about))
        }
    );
    r
}

#[cfg(test)]
mod tests {
    use crate::{mods_at};

    #[test]
    fn it_works() {
        eprintln!("{:?}", mods_at("/Applications/RimWorld.app/Mods"));

    }
}