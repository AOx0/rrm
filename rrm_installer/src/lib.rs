extern crate core;

use rrm_locals::GamePath;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};

use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use std::process::exit;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "windows")]
static DEFAULT_PAGING_SOFTWARE: &str = r"C:\Windows\System32\more.com";

#[cfg(all(target_os = "macos", feature = "dev"))]
    static PROJECT_DIR: Dir = include_dir!("rrm_installer/src/steamcmd/macos");
#[cfg(all(target_os = "macos", not( feature = "dev")))]
    static PROJECT_DIR: Dir = include_dir!("src/steamcmd/macos");

#[cfg(all(target_os = "windows", feature = "dev"))]
    static PROJECT_DIR: Dir = include_dir!("rrm_installer/src/steamcmd/windows");
#[cfg(all(target_os = "windows", not( feature = "dev")))]
    static PROJECT_DIR: Dir = include_dir!("src/steamcmd/windows");

#[cfg(all(target_os = "linux", feature = "dev"))]
    static PROJECT_DIR: Dir = include_dir!("rrm_installer/src/steamcmd/linux");
#[cfg(all(target_os = "linux", not( feature = "dev")))]
    static PROJECT_DIR: Dir = include_dir!("src/steamcmd/linux");

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

    let config = home.join(".rrm").join("config");
    config.exists() && config.is_file()
}

fn config_exists(home: &Path) -> bool {
    let config = home.join(".rrm").join("config");
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
    pub with_paging: String,
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

fn run_steam_command(c: &str, config_path: &Path) {
    #[cfg(target_os = "macos")]
    let steam = config_path.join("steamcmd").join("steamcmd");

    #[cfg(target_os = "linux")]
    let steam = config_path.join("steamcmd").join("steamcmd.sh");

    #[cfg(target_os = "windows")]
    let steam = config_path.join("steamcmd").join("steamcmd.exe");

    #[cfg(target_os = "windows")]
    let try_execute_steam = std::process::Command::new(steam.as_path().to_str().unwrap())
        .args("+login anonymous {} +quit".replace("{}", c).split(" "))
        .stdin(std::process::Stdio::null())
        .spawn()
        .unwrap_or_else(|error| {
            eprintln!("Could not execute steamcmd successfully.\nError: {}", error);
            exit(1);
        })
        .wait()
        .unwrap()
        .code()
        .unwrap();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let _try_execute_steam = std::process::Command::new("env")
        .args(
            r#"HOME=PATH [] +login anonymous {} +quit"#
                .replace(
                    "PATH",
                    steam
                        .as_path()
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
                .replace("[]", steam.as_path().to_str().unwrap())
                .replace("{}", c)
                .split(" "),
        )
        .stdin(std::process::Stdio::null())
        .spawn()
        .unwrap_or_else(|error| {
            eprintln!("Could not execute steamcmd successfully.\nError: {}", error);
            exit(1);
        })
        .wait()
        .unwrap()
        .code()
        .unwrap();
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn set_permissions_for_steamcmd(path: &Path) {
    let files = path.read_dir().unwrap();

    for file in files {
        let file = file.unwrap();

        if !file.file_type().unwrap().is_dir() {
            let mut perms = fs::metadata(file.path()).unwrap().permissions();
            perms.set_mode(0o744);
            std::fs::set_permissions(file.path(), perms).unwrap();
        } else {
            set_permissions_for_steamcmd(&file.path());
        }
    }
}

impl Installer {
    fn init(path: Option<PathBuf>) -> Self {
        let home = if let Some(home) = get_home() {
            home
        } else {
            eprintln!("Something failed while getting the user's home dir");
            exit(1);
        };

        let config_file = home.join(".rrm").join("config");
        let config = if config_exists(&home) {
            config_file
        } else {
            create_config(&config_file);
            if config_exists(&home) {
                let config_path = config_file.clone();
                let config_path = config_path.as_path().parent().unwrap();
                let steamcmd_path = config_path.join("steamcmd");
                fs::create_dir(&steamcmd_path).unwrap_or_else(|err| {
                    if !steamcmd_path.is_dir() {
                        panic!("{}", err);
                    }
                });

                PROJECT_DIR.extract(steamcmd_path.as_path()).unwrap();

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                set_permissions_for_steamcmd(steamcmd_path.as_path());

                config_file
            } else {
                exit(1);
            }
        };

        let path = path.map(|path| GamePath::from(&path));

        std::env::set_current_dir(home.join(".rrm").as_path()).unwrap();

        run_steam_command("", config.parent().unwrap().to_str().unwrap().as_ref());

        Installer {
            with_paging: DEFAULT_PAGING_SOFTWARE.to_string(),
            home,
            rim_install: path,
            config,
            use_more: true,
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
                        eprintln!(
                            "Warning: Previous saved game location \"{}\" no longer exists.",
                            rim.display()
                        );
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

    pub fn set_paging_software(&mut self, value: &str) {
        let try_execute_pager = std::process::Command::new(value)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap_or_else(|error| {
                eprintln!(
                    "Could not execute pager successfully, make sure the path or name\n\
                        is correct or that it is available within the PATH.\n\
                        Error: {}",
                    error
                );
                exit(1);
            })
            .wait()
            .unwrap_or_else(|error| {
                eprintln!(
                    "Could not execute pager successfully, make sure the path or name\n\
                        is correct or that it is available within the PATH.\n\
                        Error: {}",
                    error
                );
                exit(1);
            })
            .code();

        if let Some(code) = try_execute_pager {
            if code == 0 {
                self.with_paging = value.to_string();
                self.write_config()
            }
        } else {
            eprintln!(
                "Could not execute pager successfully, make sure the path or name\n\
                        is correct or that it is available within the PATH."
            );
        }
    }
}
