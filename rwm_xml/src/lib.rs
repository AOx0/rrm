use std::path::PathBuf;

#[derive(Debug)]
struct Mod {
    pub path: PathBuf,
    pub name: String,
    pub identifier: String,
    pub version: String
}


use std::fs::File;
use std::io::{BufReader, Read};

extern crate xml;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

#[derive(Debug)]
pub struct Element {
    pub value: String,
    pub name: String
}

pub trait XMLFile {
    fn values_of(&self, keys: Vec<&str>) -> Vec<Element>;
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Element {
            value: self.value.clone(),
            name: self.name.clone()
        }
    }
}

impl XMLFile for File {
    fn values_of(&self, keys: Vec<&str>) -> Vec<Element> {
        let mut r = vec![];
        let mut record = Element {
            value: "".to_string(),
            name: "".to_string()
        };

        let mut file = BufReader::new(self);
        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents).unwrap();
        fix_common_issues(&mut contents);

        let contents = String::from_utf8(contents).unwrap();


        let parser= EventReader::from_str(&contents);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    record.name = name.to_string();
                }
                Ok(XmlEvent::Characters(value)) => {
                    if keys.contains(&&*record.name) {
                        record.value  = value;
                        r.push(record.clone());
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
                _ => {}
            }
        }

        r
    }
}

#[test]
fn test() {
    use crate::*;

    let file = File::open("/Applications/RimWorld.app/Mods/Achtung/About/Manifest.xml").unwrap();
    let r = file.values_of(vec!["version", "identifier"]);
    println!("{:?}", r);

    let file = File::open("/Applications/RimWorld.app/Mods/Area_unlocker/About/Manifest.xml").unwrap();
    let r = file.values_of(vec!["version", "identifier"]);

    println!("{:?}", r);
}

fn fix_common_issues(contents: &mut Vec<u8>) {
    rm_bom(contents);
}

fn rm_bom(contents: &mut Vec<u8>) {
    let pat: [u8; 3] = [0xEF, 0xBB, 0xBF];

    let err = contents.windows(pat.len()).position(|w| w.eq(&pat));

    match err {
        Some(i) =>  {
            for _ in 0..3 {
                contents.remove(i);
            }
        }
        _ => {}
    }
}