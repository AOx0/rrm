use crate::args::Install;
use crate::printf;
use crate::utils::*;
use async_recursion::async_recursion;
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use regex::Regex;
use rrm_locals::{FilterBy, Filtrable};
use rrm_scrap::{FlagSet, ModSteamInfo};
use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[async_recursion(?Send)]
pub async fn install(
    mut args: Install,
    i: Installer,
    d: usize,
    mut already_installed: HashSet<String>,
) {
    let inline: bool = !(args.r#mod.len() == 1 && args.r#mod.get(0).unwrap() == "None");
    if !inline {
        args.r#mod = Vec::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line == "END" {
                    break;
                } else {
                    args.r#mod.push(line)
                }
            } else {
                println!("Error al leer: {:?}", line);
            }
        }
    }

    if args.r#mod.is_empty() {
        std::process::exit(0);
    }

    use rrm_scrap::Filtrable;

    let mut to_install: Vec<ModSteamInfo> = Vec::new();
    let filter_obj = args.to_filter_obj();
    let re = Regex::new(r"[a-zA-Z/:.]+\?id=(?P<id>\d+).*").unwrap();

    for m in &args.r#mod {
        if m.chars().all(char::is_numeric) {
            if args.verbose {
                println!("Adding {} to queue", m);
            }

            to_install.push(ModSteamInfo {
                id: m.clone(),
                title: m.clone(),
                description: "".to_string(),
                author: "".to_string(),
            });
            continue;
        }

        if m.contains("steamcommunity.com") {
            let m = m.replace("\n", "").replace(" ", "");

            if let Some(id) = extract_id(&m, &re) {
                if args.verbose {
                    println!("Adding {} to queue", id);
                }

                to_install.push(ModSteamInfo {
                    id: id.clone(),
                    title: id.clone(),
                    description: "".to_string(),
                    author: "".to_string(),
                });
                continue;
            }
        }

        let mods = SteamMods::search(m).await.with_raw_display(None);

        let mut mods = if args.filter.is_some() {
            let value = if args.filter.as_ref().unwrap().is_some() {
                args.filter.as_ref().unwrap().clone().unwrap()
            } else {
                m.clone()
            };

            mods.filter_by(filter_obj, &value)
        } else {
            mods
        };

        if !mods.is_empty() {
            if args.yes {
                to_install.push(mods[0].clone());
                continue;
            }

            printf!(
                "Adding {} by {}. Want to continue? [y/n/s(select_other)]: ",
                &mods[0].title,
                &mods[0].author
            );
            let n = read();

            if n == "yes" || n == "y" {
                to_install.push(mods[0].clone());
            } else if n == "n" || n == "no" {
                continue;
            } else if n == "s"
                || n == "select_other"
                || n == "select"
                || n == "select other"
                || n == "o"
                || n == "other"
            {
                let more_mods = mods.mods.clone().to_owned();
                mods.mods = mods.mods[0..5].to_owned();
                mods.display();

                let mut already_large = false;
                loop {
                    printf!("Select the mod # to download: ");
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
                            printf!("Added {} by {}...\n", &mods[n].title, &mods[n].author);
                            break;
                        } else {
                            println!("Enter a valid number or 'm' to show more options")
                        }
                    }
                }
            }
        } else {
            if !inline || args.verbose {
                println!("No results found for {}", m);
            }

            continue;
        }
    }

    let ids: Vec<&str> = to_install.iter().map(|e| e.id.as_str()).collect();

    if d == 0 {
        printf!(
            "{}Installing mod{} ... {}",
            if args.verbose { "\n" } else { "" },
            if args.r#mod.len() > 1 { "s" } else { "" },
            if args.verbose { "\n" } else { "" }
        )
    };
    let result = crate::async_installer::install(args.clone(), &ids, i.clone()).await;

    let mut successful_ids = HashSet::new();

    for line in result.split('\n') {
        if line.contains("Success") {
            let line = line.replace("Success. Downloaded item ", "");
            let words = line.split(' ').next().unwrap();
            successful_ids.insert(words.to_string());
        }
    }

    let rim_install = i.rim_install.as_ref().unwrap();

    let mut dependencies_ids = RefCell::new(HashSet::new());

    for id in successful_ids {
        #[cfg(target_os = "windows")]
        let source = format!(
            "{}",
            i.config
                .parent()
                .unwrap()
                .join(r"steamcmd\steamapps\workshop\content\294100\")
                .join(&id)
                .display()
        );

        #[cfg(target_os = "linux")]
        let source = format!(
            "{}",
            i.config
                .parent()
                .unwrap()
                .join(r".steam/steamapps/workshop/content/294100/")
                .join(&id)
                .display()
        );

        #[cfg(target_os = "macos")]
        let source = format!(
            "{}",
            i.config
                .parent()
                .unwrap()
                .join("Library/Application Support/Steam/steamapps/workshop/content/294100")
                .join(&id)
                .display()
        );

        let destination = rim_install.path().join("Mods");

        let options = CopyOptions {
            overwrite: true,
            ..Default::default()
        };

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        for old_mod in filtered.mods {
            //dbg!(&old_mod.path);
            dir::remove(old_mod.path).unwrap();
        }

        if args.verbose {
            println!(
                "Moving \"{}\" to \"{}\"",
                source.as_str(),
                destination.to_str().unwrap_or("error")
            );
        }

        dir::move_dir(&source, &destination, &options).unwrap();

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        already_installed.insert(id.clone());

        let is_installed = filtered.mods.len() == 1 && !filtered.mods.is_empty();

        if is_installed {
            let m: Mod = filtered.mods[0].clone(); // Get the installed mod as Mod instance (read its dependencies)

            // If it does have dependencies
            if let Some(dependencies) = m.dependencies {
                // Then add the ids to the queue
                for id in dependencies {
                    if !already_installed.contains(&id) {
                        dependencies_ids.get_mut().insert(id);
                    }
                }
            }
        } else {
            eprintln!(
                "Something unexpected happened. Duplicated mod: {}",
                filtered.mods[0].steam_id
            );
        }
    }

    if d == 0 {
        printf!("Done!\n");
    };

    //dbg!(&dependencies_ids);

    if args.resolve && !dependencies_ids.get_mut().is_empty() {
        if d == 0 {
            printf!(
                "{:<30}{}",
                "Installing dependencies ... ",
                if args.verbose { '\n' } else { ' ' }
            );
        };
        args.r#mod = Vec::from_iter(dependencies_ids.get_mut().clone());
        install(args.clone(), i.clone(), d + 1, already_installed).await;
        if d == 0 {
            printf!("Done!\n");
        };
    }
}

fn read() -> String {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    n.replace("\n", "").replace(" ", "")
}
