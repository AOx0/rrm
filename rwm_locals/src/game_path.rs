use std::path::{Path, PathBuf};
use std::process::exit;

pub struct GamePath(Box<Path>);

impl GamePath {
    fn create(path: &Path) -> Self {
        let has_mods_dir = path
            .read_dir()
            .unwrap_or_else(|_| {
                eprintln!("Failed to read contents inside {}", path.display());
                exit(1);
            })
            .any(|path| path.unwrap().file_name() == "Mods");

        if path.exists() && has_mods_dir {
            GamePath(Box::from(path))
        } else if !path.exists() {
            eprintln!("The path does not exist. Make sure you input a valid one.");
            exit(1);
        } else if !has_mods_dir {
            eprintln!("Failed to read contents inside {}", path.display());
            exit(1);
        } else {
            eprintln!(
                "Unknown error when trying to create GamePath with path: {}",
                path.display()
            );
            exit(1);
        }
    }
}

impl From<&str> for GamePath {
    fn from(path: &str) -> Self {
        GamePath::create(Path::new(path))
    }
}

impl From<&Path> for GamePath {
    fn from(path: &Path) -> Self {
        GamePath::create(path)
    }
}

impl GamePath {
    pub fn path(&self) -> Box<Path> {
        self.0.clone()
    }
}

impl From<&PathBuf> for GamePath {
    fn from(path: &PathBuf) -> Self {
        GamePath::create(path.as_path())
    }
}
