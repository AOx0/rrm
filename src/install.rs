use std::cell::RefCell;
use std::collections::HashSet;
use async_recursion::async_recursion;
use rrm_locals::{FilterBy, Filtrable};
use fs_extra::dir;
use rrm_scrap::{FlagSet, ModSteamInfo};
use crate::utils::*;
use crate::args::Install;
use crate::{printf};

#[async_recursion]
pub async fn install(mut args: Install, i: Installer, d: usize, mut already_installed: HashSet<String>) {

    if args.r#mod.len() == 1 && args.r#mod.get(0).unwrap() == "None" {
        args.r#mod = Vec::new();
        loop {
            let n: String = read()
                .replace("\n", "")
                .replace(" ", "");;

            if n != "END" {
                args.r#mod.push(n);
            } else {
                break;
            }
        }
    }

    if args.r#mod.is_empty() { std::process::exit(0); }

    use rrm_scrap::Filtrable;

    let mut to_install : Vec<ModSteamInfo> = Vec::new();
    let filter_obj = args.to_filter_obj();
    for m in &args.r#mod {
        if m.chars().all(char::is_numeric) {
            to_install.push(ModSteamInfo {
                id: m.clone(),
                title: m.clone(),
                description: "".to_string(),
                author: "".to_string()
            });
            continue;
        }

        if m.contains("https://steamcommunity.com/sharedfiles/filedetails/?id=") {
            let m = m
                .replace("https://steamcommunity.com/sharedfiles/filedetails/?id=", "")
                .replace("\n", "")
                .replace(" ", "");

            to_install.push(ModSteamInfo {
                id: m.clone(),
                title: m.clone(),
                description: "".to_string(),
                author: "".to_string()
            });
            continue;
        }

        let mods = SteamMods::search(&m)
            .await
            .with_raw_display(None);

        let mut mods = if args.filter.is_some() {
            let value = if args.filter.as_ref().unwrap().is_some() {
                args.filter.as_ref().unwrap().clone().unwrap()
            } else {
                m.clone()
            };

            mods.filter_by(filter_obj, &value)
        } else { mods };

        if !mods.is_empty() {
            let mut n = "".to_string();

            if args.yes {
                to_install.push(mods[0].clone());
            } else {
                printf!("Adding {} by {}. Want to continue? [y/n/s(select_other)]: ",&mods[0].title, &mods[0].author);
                n = read();

                if n == "yes" || n == "y" || args.yes {
                    to_install.push(mods[0].clone());
                }
            }

            if n == "s" || n == "select_other" || n == "select" || n == "select other" || n == "o" || n == "other" {
                let more_mods = mods.mods.clone().to_owned();
                mods.mods = mods.mods[0..5].to_owned();
                mods.display();

                let mut already_large = false;
                loop {
                    printf!("Select which number to add to installation cart: ");
                    let n: String = read();
                    if n == "m" {
                        mods.mods = more_mods.clone();
                        if !already_large {
                            mods.display();
                        }
                        already_large = true;
                    } else {
                        let n: usize = n.parse().unwrap();
                        if n < mods.len() {
                            to_install.push(mods[0].clone());
                            printf!("Added {} by {}...\n",&mods[n].title, &mods[n].author);
                            break;
                        } else {
                            println!("Enter a valid number or 'm' to show more options")
                        }
                    }
                }
            }

        } else {
            println!("No results found");
            continue;
        }
    }

    let ids : Vec<&str> = to_install.iter().map(|e| e.id.as_str()).collect();

    if d == 0 {printf!("Installing mod{}...", if &args.r#mod.len() > &1 { "s" } else {""});};
    let (_, result) = i.install(&ids);

    let mut successful_ids = HashSet::new();

    for line in result.split("\n") {
        if line.contains("Success") {
            let line = line.replace("Success. Downloaded item ", "");
            //println!("{}", line);
            let words = line.split(" ").next().unwrap();
            successful_ids.insert(words.to_string());
        }
    }

    let rim_install = i.rim_install.as_ref().unwrap();

    let mut dependencies_ids = RefCell::new(HashSet::new());

    for id in successful_ids {


        #[cfg(target_os="windows")]
            let source = format!("{}", i.config.parent().unwrap().join(r"steamcmd\steamapps\workshop\content\294100\").join(&id).display());

        #[cfg(target_os="linux")]
            let source = format!("{}", i.config.parent().unwrap().join(r".steam/steamapps/workshop/content/294100/").join(&id).display());

        #[cfg(target_os="macos")]
            let source = format!("{}", i.config.parent().unwrap().join("Library/Application Support/Steam/steamapps/workshop/content/294100").join(&id).display());

        let destination = rim_install.path().join("Mods");

        let mut options = dir::CopyOptions::default();
        options.overwrite = true;

        let installed_mods =  GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        for old_mod in filtered.mods {
            //dbg!(&old_mod.path);
            dir::remove(old_mod.path).unwrap();
        }

        dir::move_dir(&source, &destination, &options ).unwrap();

        let installed_mods =  GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        already_installed.insert(id.clone());

        if filtered.mods.len() == 1 && !filtered.mods.is_empty() {
            let m = filtered.mods[0].clone();
            if let Some(dependencies) = m.dependencies {
                for id in  dependencies {
                    if !already_installed.contains(&id) {
                        dependencies_ids.get_mut().insert(id);
                    }
                }
            }
        } else {
            eprintln!("There's a duplicated mod. {}", filtered.mods[0].steam_id );
        }
    }

    if d == 0 { printf!("Done!\n"); };

    //dbg!(&dependencies_ids);

    if args.resolve && dependencies_ids.get_mut().len() != 0 {
        if d == 0 { printf!("Installing dependencies recursively..."); };
        args.r#mod = Vec::from_iter(dependencies_ids.get_mut().clone());
        install(args.clone(), i.clone(), d + 1, already_installed).await;
        if d == 0 { printf!("Done!\n"); };
    }
}


fn read() -> String {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();

    let n = n.replace("\n", "").replace(" ", "");
    n
}