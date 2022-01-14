use std::path::Path;

#[cfg(target_os = "macos")]
pub const RW_DEFAULT_PATH: [&'static str; 2] = [
    r"/Applications/RimWorld.app/",
    r"~/Library/Application Support/Steam/steamapps/common/RimWorld"
];
#[cfg(target_os = "linux")]
pub const RW_DEFAULT_PATH: [&'static str; 3] = [
    r"~/GOG Games/RimWorld",
    r"~/.steam/steam/SteamApps/common/",
    r"~/.local/share/Steam/steamapps/common/RimWorld"
];
#[cfg(target_os = "windows")]
pub const RW_DEFAULT_PATH: [&'static str; 1] = [
    r"C:\Program Files (x86)\Steam\steamapps\common"
];

pub fn dir_exists(path: &str) -> bool {
    let dir = Path::new(path);
    dir.exists() && dir.is_dir()
}