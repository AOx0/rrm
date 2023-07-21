extern crate xml;

use std::fs::File;
use std::io::{BufReader, Read};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub struct Element {
    pub value: String,
    pub name: String,
}

pub trait XMLFile {
    fn values_of(&self, keys: &[&str]) -> Vec<Element>;
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Element {
            value: self.value.clone(),
            name: self.name.clone(),
        }
    }
}

impl XMLFile for File {
    fn values_of(&self, keys: &[&str]) -> Vec<Element> {
        let mut r = vec![];
        let mut record = Element {
            value: "".to_string(),
            name: "".to_string(),
        };

        let mut file = BufReader::new(self);
        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents).unwrap();
        fix_common_issues(&mut contents);

        let contents = String::from_utf8(contents).unwrap();

        let parser = EventReader::from_str(&contents);
        let mut depth = 0;
        let mut dep = false;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    depth += 1;
                    record.name = name.to_string();
                    if record.name.as_str() == "modDependencies" {
                        dep = true
                    };
                }
                Ok(XmlEvent::Characters(value)) => {
                    if keys.contains(&&*record.name) && ([0, 1, 2].contains(&depth)) {
                        record.value = value;
                        r.push(record.clone());
                    } else if dep && record.name == "steamWorkshopUrl" {
                        let value = value.to_string();

                        let value: Vec<&str> = value
                            .split("")
                            .filter(|c| {
                                ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"].contains(c)
                            })
                            .collect();

                        record.value = value.join("");
                        r.push(record.clone());
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if record.name.as_str() == "modDependencies" {
                        dep = false
                    };
                    depth -= 1;
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
    let r = file.values_of(&["version", "identifier"]);
    println!("{:?}", r);

    let file = File::open("/Applications/RimWorld.app/Mods/Achtung/About/About.xml").unwrap();
    let r = file.values_of(&["version", "identifier"]);
    println!("{:?}", r);

    let file =
        File::open("/Applications/RimWorld.app/Mods/Area_unlocker/About/Manifest.xml").unwrap();
    let r = file.values_of(&["version", "identifier"]);

    println!("{:?}", r);
}

fn fix_common_issues(contents: &mut Vec<u8>) {
    rm_bom(contents);
}

fn rm_bom(contents: &mut Vec<u8>) {
    let pat: [u8; 3] = [0xEF, 0xBB, 0xBF];

    let err = contents.windows(pat.len()).position(|w| w.eq(&pat));

    if let Some(i) = err {
        for _ in 0..3 {
            contents.remove(i);
        }
    }
}
