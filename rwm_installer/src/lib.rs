extern crate core;

use std::fs;
use rwm_locals::GamePath;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

#[cfg(target_os = "windows")]
static DEFAULT_PAGING_SOFTWARE: &str = r"C:\Windows\System32\more.com";

#[cfg(any(target_os = "linux", target_os = "macos"))]
static DEFAULT_PAGING_SOFTWARE: &str = r"more";


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn config_does_exists() -> bool {
    let home = if let Some(home) = get_home() {
        home
    } else {
        eprintln!("Something failed while getting the user's home dir");
        exit(1);
    };

    let config = home.join(".rwm").join("config");
    config.exists() && config.is_file()
}

fn config_exists(home: &Path) -> bool {
    let config = home.join(".rwm").join("config");
    config.exists() && config.is_file()
}

fn get_home() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
        return Some(std::env::current_dir().unwrap());

    #[cfg(not(feature = "dev"))]
        if let Some(user_dirs) = directories::UserDirs::new() {
            let home: &Path = user_dirs.home_dir();
            Some(home.to_path_buf())
        } else {
            None
        }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Installer {
    pub config: PathBuf,
    pub home: PathBuf,
    pub rim_install: Option<GamePath>,
    pub use_more: bool,
    pub with_paging: String
}

fn create_config(at: &Path) {
    fs::create_dir(at.parent().unwrap()).unwrap_or_else(|err| {
        if !at.parent().unwrap().is_dir() {
            panic!("{}", err);
        }
    });
    File::create(at).unwrap_or_else(|err| {
        eprintln!("Something happened while trying to create {}", at.display());
        eprintln!("Error: {}", err);
        exit(1);
    });
}

impl Installer {
    fn init(path: Option<PathBuf>) -> Self {
        let home = if let Some(home) = get_home() {
            home
        } else {
            eprintln!("Something failed while getting the user's home dir");
            exit(1);
        };

        let config_file = home.join(".rwm").join("config");
        let config = if config_exists(&home) {
            config_file
        } else {
            create_config(&config_file);
            if config_exists(&home) {
                config_file
            } else {
                exit(1);
            }
        };

        let path = path.map(|path| GamePath::from(&path));

        Installer {
            with_paging: DEFAULT_PAGING_SOFTWARE.to_string(),
            home,
            rim_install: path,
            config,
            use_more: true
        }
    }

    fn init_with_path(rim_path: GamePath) -> Self {
        let rim_install = rim_path.path().to_path_buf();
        Installer::init(Some(rim_install))
    }

    pub fn write_config(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();

        let mut config = OpenOptions::new()
            .append(false)
            .create(true)
            .write(true)
            .truncate(true)
            .read(false)
            .open(&self.config)
            .unwrap_or_else(|err| {
                eprintln!("Failed to open config file at {}", &self.config.display());
                eprintln!("Error: {}", err);
                exit(1);
            });

        config.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_config(path: &Path) -> serde_json::Result<Installer> {
        let file = File::open(path).unwrap_or_else(|err| {
            eprintln!("Failed to open config file at {}", path.display());
            eprintln!("Error: {}", err);
            exit(1);
        });
        let mut buf = BufReader::new(file);
        let mut res: Vec<u8> = vec![];
        buf.read_to_end(&mut res).unwrap();

        serde_json::from_slice(&res)
    }

    pub fn new(with_path: Option<GamePath>) -> Option<Self> {
        let installer = if let Some(path) = with_path {
            Installer::init_with_path(path)
        } else {
            let installer = Installer::init(None);
            let old_config = Installer::load_config(&installer.config);

            if let Ok(mut i) = old_config {
                if i.rim_install.as_ref().is_some() {
                    let rim = i.rim_install.as_ref().unwrap().path();
                    if rim.exists() {
                        i
                    } else {
                        eprintln!("Warning: Previous saved game location \"{}\" no longer exists.",
                                 rim.display());
                        i.rim_install = None;
                        i
                    }
                } else {
                    i
                }
            } else {
                installer
            }
        };

        installer.write_config();
        Some(installer)
    }

    pub fn set_more_value(&mut self, value: bool) {
        self.use_more = value;
        self.write_config();
    }

    pub fn set_path_value(&mut self, value: PathBuf) {
        self.rim_install = Some(GamePath::from(value.as_path()));
        self.write_config();
    }
}
