mod game_path;
mod mod_obj;
mod mod_paths;

pub use game_path::*;
pub use mod_obj::*;
pub use mod_paths::*;
use std::ops::{Deref};
use fuzzy_matcher::*;

pub use flagset::*;
use std::process::{exit, Stdio};
use std::io::Write;

pub type Mods = Vec<Mod>;

#[derive(Clone)]
pub struct GameMods {
    mods: Mods,
    pub biggest_name_size: usize,
    display_type: Option<DisplayType>,
}

impl GameMods {

    pub fn new() -> Self {
        GameMods {
            mods: Vec::new(),
            biggest_name_size: 0,
            display_type: None
        }
    }

    pub fn with_display(self, t: DisplayType) -> Self {
        let mut s = self;
        s.display_type = Some(t);
        s
    }

    pub fn gen_display(&self) -> String {
        let mut result = "".to_string();

        let d_type = self.display_type.as_ref().unwrap_or_else(|| {
            eprintln!("Error, make sure to set display_type to a variant of DisplayType");
            exit(1);
        });

        if let DisplayType::Short = d_type {
            result.push_str( &format!("{}\n", Mod::gen_headers(self.biggest_name_size)));
        }

        self.mods
            .iter()
            .for_each(
            |m|
                result.push_str(&format!("{}\n", m.gen_display(d_type, self.biggest_name_size)))
            );

        result
    }

    pub fn more_display(&self, with_pager: &str) {
        let output = self.gen_display();

        let mut more = std::process::Command::new(with_pager)
            .stdin(Stdio::piped())
            .spawn().unwrap();

        let more_stdin = more.stdin.as_mut().unwrap();
        more_stdin.write_all(output.as_bytes()).unwrap_or_else(|err| {
            eprintln!("Something went wrong while writing contents to `more`.\n\
            Error: {err}")
        } );

        more.wait().unwrap();
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

flags! {
    pub enum FilterBy: u8 {
        Author  = 0b00001,
        Name    = 0b00010,
        Version = 0b00100,
        SteamID = 0b01000,
        None    = 0b10000,
        All = (FilterBy::Author | FilterBy::Name | FilterBy::Version | FilterBy::SteamID).bits(),
    }
}

pub trait Filtrable<T: flagset::Flags>: Sized {
    fn filter_by(&self, filter: FlagSet<T>, value: &str) -> Self;
}

impl Filtrable<FilterBy> for GameMods {
    fn filter_by(&self, filter: FlagSet<FilterBy>, value: &str) -> Self {
        use FilterBy::*;

        let mut filtered = GameMods::new();
        let mods: Vec<Mod> = self.mods.clone();

        let matcher = skim::SkimMatcherV2::default();

        filtered.display_type = self.display_type;

        mods
            .into_iter()
            .for_each(|m| {
                let result =
                    {
                        (if filter.contains(All) || filter.contains(Name) || filter.contains(Name) {
                            matcher.fuzzy_match(&m.name, &value).is_some()
                        } else {
                            false
                        }) || (if filter.contains(Author) || filter.contains( All) {
                            matcher.fuzzy_match(&m.author, &value).is_some()
                        } else {
                            false
                        }) || (if filter.contains(Version) || filter.contains( All) {
                            matcher
                                .fuzzy_match(&m.version.clone().unwrap_or_else(|| "".to_string()), &value)
                                .is_some()
                        } else {
                            false
                        }) || (if filter.contains(SteamID) || filter.contains( All) {
                            matcher.fuzzy_match(&m.steam_id, &value).is_some()
                        } else {
                            false
                        })
                    };

                if result {
                    if m.name.len() > filtered.biggest_name_size {
                        filtered.biggest_name_size = m.name.len();
                    }

                    filtered.mods.push(m);
                };
            });


        filtered
    }
}