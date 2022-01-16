mod mod_obj;
mod mod_paths;
mod game_path;

pub use mod_obj::*;
pub use mod_paths::*;
pub use game_path::*;

use std::process::exit;
use std::slice::Iter;

pub type Mods = Vec<Mod>;

pub struct GameMods {
    mods: Mods,
    display_type: Option<DisplayType>
}

impl GameMods {
    pub fn with_display(self, t: DisplayType) -> Self {
        let mut s = self;
        s.display_type = Some(t);
        s
    }

    pub fn display(&self) {
        let d_type = self.display_type.as_ref().unwrap_or_else(|| {
            println!("Error, make sure to set display_type to a variant of DisplayType");
            exit(1);
        });

        if let DisplayType::Short =  d_type { println!("{}", Mod::gen_headers()); }

        self.mods.iter().for_each(|m| {
            m.display(d_type)
        })
    }

    pub fn mods_iter(&mut self) -> Iter<Mod> {
        self.mods.iter()
    }
}

impl From<&str> for GameMods {
    fn from(path: &str) -> Self {
        let game_path: GamePath = GamePath::from(path);
        let mods: Mods = mods_at(&game_path.path().join("Mods")).parse();

        GameMods{
            mods,
            display_type: None
        }
    }
}
