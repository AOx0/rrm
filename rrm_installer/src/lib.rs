extern crate core;

use rrm_locals::GamePath;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};


use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use std::process::exit;

use directories::UserDirs;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "windows")]
static DEFAULT_PAGING_SOFTWARE: &str = r"C:\Windows\System32\more.com";

#[cfg(target_os = "macos")]
static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/steamcmd/macos");

#[cfg(target_os = "windows")]
static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/steamcmd/windows");

#[cfg(target_os = "linux")]
static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/steamcmd/linux");

#[cfg(any(target_os = "linux", target_os = "macos"))]
static DEFAULT_PAGING_SOFTWARE: &str = r"more";

pub fn get_or_create_config_dir() -> PathBuf {
    if let Some(path) = env_var_config("XDG_CONFIG_HOME")
        .or_else(|| env_var_config("RRM_CONFIG_HOME"))
        .or_else(|| env_var_config("CONFIG_HOME"))
    {
        return path;
    }

    let config_dir = UserDirs::new().unwrap().home_dir().join(".config");
    if config_dir.exists() {
        let config_dir = config_dir.join("rrm");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).unwrap();
        }
        return config_dir.to_path_buf();
    }

    let config_dir = UserDirs::new().unwrap().home_dir().join(".rrm");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).unwrap();
    }

    config_dir.to_path_buf()
}

fn env_var_config(var: &'static str) -> Option<PathBuf> {
    std::env::var(var).ok().map(|env_config_dir| {
        let env_config_dir = PathBuf::from(env_config_dir).join("rrm");
        if !env_config_dir.exists() {
            fs::create_dir_all(&env_config_dir).unwrap();
        }
        env_config_dir
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Installer {
    pub rim_install: Option<GamePath>,
    pub use_more: bool,
    pub with_paging: String,
}

pub fn run_steam_command(c: &str, config_path: &Path, count: usize) -> String {
    let steam = get_steamcmd_path(config_path);

    #[cfg(target_os = "windows")]
    let out = std::process::Command::new(steam.as_path().to_str().unwrap())
        .args("+login anonymous {} +quit".replace("{}", c).split(" "))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .unwrap_or_else(|error| {
            eprintln!("Could not execute steamcmd successfully.\nError: {}", error);
            exit(1);
        });

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let out = std::process::Command::new("env")
        .args(
            r#"HOME=PATH [] +login anonymous {} +quit"#
                .replace(
                    "PATH",
                    config_path.as_os_str().to_str().unwrap(),
                )
                .replace("[]", steam.as_path().to_str().unwrap())
                .replace("{}", c)
                .split(' '),
        )
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .unwrap_or_else(|error| {
            eprintln!("Could not execute steamcmd successfully.\nError: {}", error);
            exit(1);
        });

    let out = String::from_utf8(out.clone().stdout).unwrap();

    if c.contains("+workshop_download_item 294100")
        && out.contains("Connecting anonymously to Steam Public...OK")
        && out.contains("Waiting for client config...OK")
        && out.contains("Waiting for user info...OK")
    {
        out
    } else if c.contains("+workshop_download_item 294100") {
        run_steam_command(c, config_path, count + 1)
    } else if count == 5 {
        "Error: Failed to install".to_string()
    } else {
        run_steam_command(c, config_path, count + 1)
    }
}

pub fn get_steamcmd_path(config_path: &Path) -> PathBuf {
    #[cfg(target_os = "macos")]
    return config_path.join("steamcmd").join("steamcmd");

    #[cfg(target_os = "linux")]
    return config_path.join("steamcmd").join("steamcmd.sh");

    #[cfg(target_os = "windows")]
    return config_path.join("steamcmd").join("steamcmd.exe");
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
        let config_dir = get_or_create_config_dir();
        let config_file = config_dir.join("config");
        if !config_file.exists() {
            File::create(&config_file).unwrap();

            let steamcmd_path = config_dir.join("steamcmd");
            fs::create_dir(&steamcmd_path).unwrap_or_else(|err| {
                if !steamcmd_path.is_dir() {
                    panic!("{}", err);
                }
            });

            // TODO: Install from https://steamcdn-a.akamaihd.net/client/installer/steamcmd
            PROJECT_DIR.extract(steamcmd_path.as_path()).unwrap();

            #[cfg(any(target_os = "macos", target_os = "linux"))]
            set_permissions_for_steamcmd(steamcmd_path.as_path());

            println!("Installing steamcmd...");
            run_steam_command("", &config_dir, 1);
            println!("Done!");
        }

        let path = path.map(|path| GamePath::from(&path));

        std::env::set_current_dir(&config_dir).unwrap();

        Installer {
            with_paging: DEFAULT_PAGING_SOFTWARE.to_string(),
            rim_install: path,
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
            .open(get_or_create_config_dir().join("config"))
            .unwrap_or_else(|err| {
                eprintln!(
                    "Failed to open config file at {}",
                    &get_or_create_config_dir().join("config").display()
                );
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
            let mut installer = Installer::load_config(&get_or_create_config_dir().join("config"))
                .unwrap_or(installer);

            if let Some(ref rim_path) = installer.rim_install {
                if !rim_path.path().exists() {
                    eprintln!(
                        "Warning: Previous saved game location \"{}\" no longer exists.",
                        rim_path.path().display()
                    );
                    installer.rim_install = None;
                }
            }
            installer
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

    pub fn install_sync(&self, c: &[&str]) -> (bool, String) {
        let to_install = Self::gen_install_string(&c);
        let a: String = run_steam_command(&to_install, &get_or_create_config_dir(), 1);
        (a.contains("Success. Downloaded item"), a)
    }

    pub fn gen_install_string(c: &&[&str]) -> String {
        
        " +workshop_download_item 294100 ".to_string()
            + &c.join(" +workshop_download_item 294100 ")
    }

    pub fn get_steamcmd_path(&self) -> PathBuf {
        get_steamcmd_path(&get_or_create_config_dir())
    }

    pub fn run_steam_command(&self, c: &str, count: usize) -> String {
        run_steam_command(c, &get_or_create_config_dir(), count)
    }
}
