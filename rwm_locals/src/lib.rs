mod game_path;
mod mod_obj;
mod mod_paths;

pub use game_path::*;
pub use mod_obj::*;
pub use mod_paths::*;
use std::ops::Deref;

use std::process::exit;

pub type Mods = Vec<Mod>;

#[derive(Clone)]
pub struct GameMods {
    mods: Mods,
    pub biggest_name_size: usize,
    display_type: Option<DisplayType>,
}

impl GameMods {
    pub fn with_display(self, t: DisplayType) -> Self {
        let mut s = self;
        s.display_type = Some(t);
        s
    }

    pub fn display(&self) {
        let d_type = self.display_type.as_ref().unwrap_or_else(|| {
            eprintln!("Error, make sure to set display_type to a variant of DisplayType");
            exit(1);
        });

        if let DisplayType::Short = d_type {
            println!("{}", Mod::gen_headers(self.biggest_name_size));
        }

        self.mods
            .iter()
            .for_each(|m| m.display(d_type, self.biggest_name_size))
    }
}

impl Deref for GameMods {
    type Target = Mods;

    fn deref(&self) -> &Self::Target {
        &self.mods
    }
}

impl From<&str> for GameMods {
    fn from(path: &str) -> Self {
        let game_path: GamePath = GamePath::from(path);
        GameMods::from(game_path)
    }
}

impl From<GamePath> for GameMods {
    fn from(path: GamePath) -> Self {
        let (mods, biggest) = mods_at(&path.path().join("Mods")).parse();

        GameMods {
            mods,
            display_type: None,
            biggest_name_size: biggest,
        }
    }
}
