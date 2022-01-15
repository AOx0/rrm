use std::process::exit;
use reqwest::Response;
use lazy_regex::*;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

async fn get_contents(url: &str) -> String {
    let resp: Response = reqwest::get(url)
        .await.unwrap_or_else(|err| {
            eprintln!("{}", capitalize(&err.to_string()));
            exit(1);
        }
    );

    let resp: String = resp.text().await.unwrap_or_else(|err| {
        eprintln!("{}", capitalize(&err.to_string()));
        exit(1);
    });

    resp
}

pub type ModsSteamInfo = Vec<ModSteamInfo>;

pub async fn look_for_mod(mod_name: &str) {
    use scraper::{Html, Selector};

    let contents: String = get_contents(
        &r#"https://steamcommunity.com/workshop/browse/?appid=294100&searchtext="URL""#
            .replace("URL", mod_name)
    ).await;

    let mut mods_steam_info: ModsSteamInfo = vec![];

    let contents: Html = Html::parse_document(&contents);
    let script: Selector = Selector::parse("#profileBlock > div > div.workshopBrowseItems > script").unwrap();

    for element in contents.select(&script) {

        let m  = html_escape::decode_html_entities(element.inner_html().as_str()).to_string();

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
        }).to_string();

        //Get rid of \\n \\t \\r, etc.
        let s = regex_replace_all!(r#"(\\.)"#, &s, |_, _| {
            "".to_string()
        });

        //Get rid of multiple contiguous spaces
        let mut m = regex_replace_all!(r#"( +)"#, &s, |_, _| {
            " ".to_string()
        }).to_string();

        //Remove ); from the end
        m.remove(m.len()-1);
        m.remove(m.len()-1);

        let ms: Vec<String> = m.split(",").collect::<Vec<&str>>().into_iter().map(|m| {
            m.trim().to_string()
        }).collect();

        let mut msf = vec![];

        ms.into_iter().for_each(|m| {
            if m.contains("\"id") || m.contains("\"title") || m.contains("\"description"){
                msf.push(m.replace("\"", ""));
            }
        });

         mods_steam_info.push( ModSteamInfo {
            id: msf[0].replace("{id:", ""),
            title: msf[1].replace("title:", ""),
            description: msf[2].replace("description:", ""),
            author: "".to_string()
        });
    }

    let author: Selector = Selector::parse("#profileBlock > div > div.workshopBrowseItems > div > div.workshopItemAuthorName.ellipsis > a").unwrap();

    let mut i = 0;
    for element in contents.select(&author) {

        mods_steam_info[i].author = element.inner_html().to_string();

        println!("{}", mods_steam_info[i]);

        i+=1;
    }

}

#[derive(Default)]
struct ModSteamInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: String
}

impl std::fmt::Display for ModSteamInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "\
                Title: {} [ID: {}]\n\
                Author: {}\n\
                Description: {}\n\
        ", self.title, self.id, self.author, self.description
        )
    }
}