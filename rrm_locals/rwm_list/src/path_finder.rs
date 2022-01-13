use std::path::{Path, PathBuf};
pub use path_absolutize::Absolutize;

fn list_p(path: &Path) -> Vec<PathBuf> {
    let contents = path.absolutize().unwrap().read_dir().unwrap();
    let mut result = vec![];
    for e  in contents {
        result.push(e.unwrap().path());
    }
    result
}

fn list_b(buf: &PathBuf) -> Vec<PathBuf> {
    list_p(buf.as_path())
}

fn list_path_abouts(path: &str)  -> Vec<PathBuf> {
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
    pub path: PathBuf
}

pub fn get_mods(about_dir: &PathBuf) -> Vec<ModPaths> {
    let mut mod_files = vec![];
    list_b(about_dir)
        .iter()
        .for_each(|path| {
            let m = ModPaths {
                about: if path.file_name().unwrap() == "About.xml" { Some(PathBuf::from(path)) } else { None },
                manifest: if path.file_name().unwrap() == "Manifest.xml" { Some(PathBuf::from(path)) } else { None },
                path: PathBuf::from(path.parent().unwrap())
            };

            if m.about.is_some() || m.manifest.is_some() {
                mod_files.push(m);
            }

        }
        );

    mod_files
}

#[cfg(test)]
mod tests {
    use crate::path_finder::{get_mods, list_path_abouts};

    #[test]
    fn it_works() {
        eprintln!("{}", list_path_abouts("/Applications/RimWorld.app/Mods").len());
        list_path_abouts("/Applications/RimWorld.app/Mods").iter().for_each(
            |about| get_mods(about).iter().for_each(
                |about| println!("{:?}", about)
            )
        );
    }
}