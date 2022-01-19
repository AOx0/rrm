pub use rwm_locals::{DisplayType, GameMods, GamePath, Mod, Mods};
pub use rwm_scrap::*;
pub use std::path::{Path, PathBuf};
use std::process::exit;
use rwm_installer::Installer;

#[cfg(target_os = "macos")]
pub const RW_DEFAULT_PATH: [&str; 2] = [
    r"/Applications/RimWorld.app/",
    r"~/Library/Application Support/Steam/steamapps/common/RimWorld/RimWorldMac.app/",
];
#[cfg(target_os = "linux")]
pub const RW_DEFAULT_PATH: [&str; 3] = [
    r"~/GOG Games/RimWorld",
    r"~/.steam/steam/SteamApps/common/",
    r"~/.local/share/Steam/steamapps/common/RimWorld",
];
#[cfg(target_os = "windows")]
pub const RW_DEFAULT_PATH: [&str; 2] = [
    r"C:\Program Files (x86)\Steam\steamapps\common\RimWorld",
    r"C:\Program Files\Steam\steamapps\common\RimWorld",
];

pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub const LIST_DESCRIPTION: &str = "List installed Mods in Path/To/RimWorld/Mods/";
#[cfg(target_os = "windows")]
pub const LIST_DESCRIPTION: &str = r#"List installed Mods in C:\Path\To\RimWorld\Mods"#;

pub fn try_get_path(game_path: Option<&Path>) -> Installer {
    if let Some(game_path) = game_path {

        if dir_exists(game_path) {
            Installer::new(Some(GamePath::from(game_path))).unwrap()
        } else {
            eprintln!(
                "Error: \"{}\" is not a valid RimWorld installation path.",
                game_path.display()
            );
            exit(1);
        }
    } else if let Some(installer) = Installer::new(None) {
        return installer;
    } else {
        let mut result = None;
        RW_DEFAULT_PATH.into_iter().for_each(|path| {
            if dir_exists(&PathBuf::from(path)) {
                result = Some(Installer::new(Some(GamePath::from(path)))).unwrap()
            }
        });

        result.unwrap_or_else(|| {
            eprintln!(
                "\
            Error: Unable to find RimWorld installation path.\n\
            Try specifying the path:\n\
            \trwm list -g <GAME_PATH>        <--- Like this\
        "
            );
            exit(1);
        })
    }
}
