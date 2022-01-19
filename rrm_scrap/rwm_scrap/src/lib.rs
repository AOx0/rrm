use lazy_regex::*;
use reqwest::Response;
use rwm_locals::{DisplayType, InfoString};
use std::io::Stdout;
use std::io::Write;
use std::process::exit;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

async fn get_contents(url: &str) -> String {
    let resp: Response = reqwest::get(url).await.unwrap_or_else(|err| {
        eprintln!("{}", capitalize(&err.to_string()));
        exit(1);
    });

    let resp: String = resp.text().await.unwrap_or_else(|err| {
        eprintln!("{}", capitalize(&err.to_string()));
        exit(1);
    });

    resp
}

pub async fn look_for_mod(mod_name: &str) -> (Vec<ModSteamInfo>, usize) {
    use scraper::{Html, Selector};

    let contents: String = get_contents(
        &r#"https://steamcommunity.com/workshop/browse/?appid=294100&searchtext="URL""#
            .replace("URL", mod_name),
    )
    .await;

    let mut mods_steam_info: Vec<ModSteamInfo> = vec![];

    let contents: Html = Html::parse_document(&contents);
    let script: Selector =
        Selector::parse("#profileBlock > div > div.workshopBrowseItems > script").unwrap();

    for element in contents.select(&script) {
        let m = html_escape::decode_html_entities(element.inner_html().as_str()).to_string();

        //Get rid of unused characters in description.
        let m: String = m
            .replace("\\/", "/")
            .replace(r"<br />", " ")
            .replace("SharedFileBindMouseHover( ", "");

        //Replace \uXXXX to its actual character
        let s = regex_replace_all!(r#"\\u(.{4})"#, &m, |_, num: &str| {
            let num: u32 = u32::from_str_radix(num, 16).unwrap();
            let c: char = std::char::from_u32(num).unwrap();
            c.to_string()
        })
        .to_string();

        //Get rid of \\n \\t \\r, etc.
        let s = regex_replace_all!(r#"(\\.)"#, &s, |_, _| { "".to_string() });

        //Get rid of multiple contiguous spaces
        let mut m = regex_replace_all!(r#"( +)"#, &s, |_, _| { " ".to_string() }).to_string();

        //Remove ); from the end
        m.remove(m.len() - 1);
        m.remove(m.len() - 1);

        let mut msf = vec![];

        m.split(',').into_iter().for_each(|m| {
            if m.contains("\"id") || m.contains("\"title") || m.contains("\"description") {
                msf.push(m.trim().replace("\"", ""));
            }
        });

        mods_steam_info.push(ModSteamInfo {
            id: msf[0].replace("{id:", ""),
            title: msf[1].replace("title:", ""),
            description: msf[2].replace("description:", ""),
            author: "".to_string(),
        });
    }

    let author: Selector = Selector::parse("#profileBlock > div > div.workshopBrowseItems > div > div.workshopItemAuthorName.ellipsis > a").unwrap();
    let mut size: usize = 0;

    for (i, element) in contents.select(&author).enumerate() {
        mods_steam_info[i].author = element.inner_html().to_string();
        if mods_steam_info[i].title.len() > size {
            size = mods_steam_info[i].title.len();
        }
    }

    (mods_steam_info, size)
}

#[derive(Default)]
pub struct ModSteamInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: String,
}

impl ModSteamInfo {
    fn gen_headers(size: usize) -> String {
        "".to_string()
            .add_s(format!("{:>15}", "Steam ID"))
            .add_s(format!("   {:<size$}", "Name", size = size))
            .add_s(format!("   {:<20}", "Uploader"))
            .add_s(format!("\n{:>15}", "--------"))
            .add_s(format!("   {:<size$}", "--------", size = size))
            .add_s(format!("   {:<20}", "--------"))
    }

    pub fn gen_large(&self) -> String {
        "".to_string()
            .add_s(format!("Name     : {}\n", self.title))
            .add_s(format!("Steam ID : {}\n", self.id))
            .add_s(format!("Author   : {}\n", self.author))
            .add_s(format!("Description: {}\n", self.description))
    }

    pub fn gen_short(&self, biggest_name: usize) -> String {
        "".to_string()
            .add_s(format!("{:>15}", self.id))
            .add_s(format!("   {:<size$}", self.title, size = biggest_name))
            .add_s(format!("   {:<20}", self.author))
    }

    pub fn display(&self, form: &DisplayType, biggest_name: usize) {
        let mut f: Stdout = std::io::stdout();

        if let DisplayType::Long = form {
            writeln!(f, "{}", self.gen_large()).unwrap()
        } else {
            writeln!(f, "{}", self.gen_short(biggest_name)).unwrap()
        }
    }
}

pub struct SteamMods {
    pub mods: Vec<ModSteamInfo>,
    pub biggest_name_size: usize,
    pub display_type: Option<DisplayType>,
}

impl SteamMods {
    pub async fn search(m: &str) -> Self {
        let (mods, biggest_name_size) = look_for_mod(m).await;

        SteamMods {
            mods,
            biggest_name_size,
            display_type: None,
        }
    }

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
            println!("{}", ModSteamInfo::gen_headers(self.biggest_name_size));
        }

        self.mods
            .iter()
            .for_each(|m| m.display(d_type, self.biggest_name_size))
    }
}
