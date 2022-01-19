use std::env::current_dir;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use directories::UserDirs;
use rwm_locals::GamePath;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn config_exists(home: &Path) -> bool {
    let config = home.join(".rwm_config");
    config.exists() && config.is_file()
}

fn get_home() -> Option<PathBuf> {
    if let Some(user_dirs) = UserDirs::new() {
        let home: &Path = user_dirs.home_dir();
        #[cfg(feature = "dev")]
            return Some(current_dir().unwrap());
        #[cfg(not(feature = "dev"))]
            return Some(home.to_path_buf());
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Installer {
    pub config: PathBuf,
    pub home: PathBuf,
    pub rim_install: Option<PathBuf>
}

fn create_config(at: &Path) {
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

        let config_file = home.join(".rwm_config");
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

        Installer {
            home, rim_install: path, config
        }
    }

    fn init_with_path(rim_path: GamePath) -> Self {
        let rim_install = rim_path.path().to_path_buf();
        Installer::init(Some(rim_install))
    }

    pub fn write_config(&self, ) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut config = OpenOptions::new()
            .append(false)
            .create(true)
            .write(true)
            .read(false)
            .open(&self.config).unwrap_or_else(|err| {
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

    pub fn new(with_path: Option<GamePath>) -> Self {

        let installer = if let Some(path)  = with_path {
            Installer::init_with_path(path)
        } else {
            let installer = Installer::init(None);
            let old_config = Installer::load_config(&installer.config);

            if let Ok(installer) = old_config {
                installer
            } else {
                installer
            }
        };

        installer.write_config();
        installer

    }
}