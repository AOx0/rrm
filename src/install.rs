use crate::args::{Install, InstallingOptions};
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
use text_io::try_read;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;


#[cfg(target_os = "windows")]
const PATH: &str = r"steamcmd\steamapps\workshop\content\294100\"; 

#[cfg(target_os = "linux")]
const PATH: &str = r".steam/steamapps/workshop/content/294100/";

#[cfg(target_os = "macos")]
const PATH: &str = r"Library/Application Support/Steam/steamapps/workshop/content/294100";

fn clear_leftlovers(at: &Path, args: &Install) {
    if at.is_dir() && at.exists() {
        if args.is_verbose() {
            log!(Warning: "Attempting to remove leftlovers at {}", at.display() );
        }
        for entry in at.read_dir().unwrap() {
            let entry = entry.unwrap();
            if args.is_debug() {
                log!(Warning: "Removig leftlover {}", entry.path().display() );
            }
            if entry.path().is_dir() {
                std::fs::remove_dir_all(entry.path()).unwrap();
            } else {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }
    }
} 

#[async_recursion(?Send)]
pub async fn install(
    mut args: Install,
    i: Installer,
    d: usize,
    mut already_installed: HashSet<String>,
) {
    if args.is_debug() {
        log!(Warning: "Already installed {:?}", already_installed);
    }
    let inline: bool = !(args.r#mod.len() == 1 && args.r#mod.get(0).unwrap() == "None");
    if !inline {
        args.r#mod = Vec::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line == "END" {
                    break;
                } else {
                    args.r#mod.push(line);
                }
            } else {
                log!(Error: "Could not read line {:?}", line);
            }
        }
    }

    if args.r#mod.is_empty() {
        std::process::exit(0);
    }

    use rrm_scrap::Filtrable;

    let mut to_install: Vec<ModSteamInfo> = Vec::new();
    let filter_obj = args.to_filter_obj();
    let re = Regex::new(r"[a-zA-Z/:.]+[?0-9a-zA-Z0-9/=&]+[\?\&]{1}id=(?P<id>\d+).*").unwrap();

    for m in &args.r#mod {
        if m.chars().all(char::is_numeric) {
            if m.trim().is_empty() {
                continue;
            }

            if args.is_verbose() {
                log!(Status: "Adding {} to queue", m);
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
            let m = m.replace('\n', "").replace(' ', "");

            if let Some(id) = extract_id(&m, &re) {
                if id.trim().is_empty() {
                    continue;
                }

                if args.is_verbose() {
                    log!(Status: "Adding {} to queue", id);
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
            let n = loop {
                let read: Result<String, _> = try_read!();
                if let Ok(read) = read {
                    break read;
                } else {
                    log!(Error: "Somehting wrong happened. Re-input your answer.");
                };
            };

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
                    let n: isize = loop {
                        printf!("Select the mod # to download or `-1` for more: ");
                        let num: Result<isize, _> = try_read!();
                        if let Ok(read) = num {
                            break read;
                        } else {
                            log!(Error: "Somehting wrong happened. Re-input your answer.");
                        };
                    };

                    if n < 0 {
                        mods.mods = more_mods.clone();
                        if !already_large {
                            mods.display();
                        }
                        already_large = true;
                    } else {
                        let n = n as usize;
                        if n < mods.len() {
                            to_install.push(mods[n].clone());
                            if args.is_verbose() {
                                log!(Status: "Added {} by {} (id: {})...", &mods[n].title, &mods[n].author, &mods[n].id);
                            }
                            break;
                        } else {
                            log!(Error: "Enter a valid positive index or a negative value (like -1) to show more options")
                        }
                    }
                }
            }
        } else {
            if !inline || args.is_verbose() {
                log!( Error: "No results found for {}", m);
            }

            continue;
        }
    }

    let ids: Vec<&str> = to_install.iter().map(|e| e.id.as_str()).collect();
    let ids: Vec<_> = ids.into_iter().filter(|&id| !already_installed.contains(&id.to_owned()) ).collect();

    if d == 0 {
        log!( Status:
            "Installing mod{}",
            if args.r#mod.len() > 1 { "s" } else { "" },
        )
    };

    let downloads_dir = PATH.replace("content","downloads");
    let path_downloads = PathBuf::from(&downloads_dir);
    // Remove any download leflovers
    clear_leftlovers(&path_downloads, &args);
    clear_leftlovers(&PathBuf::from(PATH), &args); 

    let (ty, ry) = channel::<bool>();
    let should_start = Arc::new(ty);
    let should_end = Arc::new(Mutex::new(false));

    
    let status_downloader = std::thread::spawn({
        let path_downloads = path_downloads.to_owned();
        let number_to_install = ids.len();
        let should_end = Arc::clone(&should_end);
        let verbose = args.is_verbose();
        move || {
            extern crate notify;

            use notify::{Watcher, watcher, DebouncedEvent};
            use std::time::Duration;

            // Wait for start signal
            let _ = ry.recv().unwrap();
            
            if verbose {
                log!(Warning: "Starting file watcher");
            }

            let mut last_printed = String::new();
            let mut d = 0;

            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();
            watcher.watch(path_downloads.parent().unwrap().to_str().unwrap(), notify::RecursiveMode::Recursive).unwrap();

            let timeout = Duration::from_secs_f32(1.0);

            loop {
                match rx.recv_timeout(timeout) {
                    Ok(event) => {
                        match event {
                            DebouncedEvent::Create(path) => {
                                let current = path.file_name().unwrap().to_str().unwrap();  
                                if let Some(n) = path.parent() {
                                    if let Some(name) = n.file_name() {
                                        let name = name.to_str().unwrap();
                                        if name == "294100" {
                                            if current != last_printed {
                                                d += 1;
                                                log!(Status: "[{1:0>3}/{2:0>3}] Dowloading {0}", current, d, number_to_install);
                                                last_printed = current.to_owned();
                                            }
                                        }
                                    } 
                                }
                            },
                            _ => {}
                        }
                    }
                    Err(_) => {
                        let end = *should_end.lock().unwrap();
                        if end == true {
                            if verbose {
                                log!(Warning: "Ending file watcher.");
                            }
                            break; 
                        }           
                    } 
                }
            }
        }
    });
 
    let result = {
        let mut result = String::new();
        let mut num = ids.len();
        let mut n = 0;
        let large_list = num > 200;

        if large_list {
            log!(Warning: "More than 200 mods. Splitting to chunks of 200 to avoid problems with SteamCMD");
        }

        while num >= 200 {
            log!(Warning: "Spawning SteamCMD with mods {}..{}", n, n+200);
            let r = crate::async_installer::install(args.clone(), &ids[n..n+200], i.clone(), Arc::clone(&should_start)).await;
            result.push_str(&r);
            num -= 200;
            n += 200;
        }

        if large_list {
            log!(Warning: "Spawning last SteamCMD with mods {}..{}", n, n+num); 
        }
        let r = crate::async_installer::install(args.clone(), &ids[n..n+num], i.clone(), Arc::clone(&should_start)).await;
        result.push_str(&r);
        result
    };

    if args.is_verbose() {
        log!(Status: "Installer finished");
    }

    {
        let mut should_end = should_end.lock().unwrap();
        *should_end = true;
    }

    status_downloader.join().unwrap();

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

    let destination = rim_install.path().join("Mods");

    if args.is_verbose() {
        log!( Status:
                "Handling & moving mods to \"{}\"",
                destination.to_str().unwrap_or("error")
        );
    }

    let mut ids: HashSet<String>  = ids.iter().map(|s| s.to_string()).collect();

    for id in successful_ids.clone() {
        ids.remove(&id);

        let source = format!(
            "{}",
            i.config
                .parent()
                .unwrap()
                .join(PATH)
                .join(&id)
                .display()
        );

        
        let options = CopyOptions {
            overwrite: true,
            ..Default::default()
        };

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        let mut ignored = false; 

        for old_mod in filtered.mods {
            //dbg!(&old_mod.path);
            if old_mod.path.starts_with('_') {
                if args.is_verbose() {
                    log!( Warning: "Ignoring {}", old_mod.path);
                }
                ignored = true;
                dir::remove(&source).unwrap();
                break;
            } else {
                dir::remove(old_mod.path).unwrap();
            }
        }

        if ignored {
            continue;
        }

        if args.verbose {
            log!( Status:
                "Moving \"{}\" to \"{}\"",
                &source,
                destination.to_str().unwrap_or("error")
            );
        }

        dir::move_dir(&source, &destination, &options).unwrap();

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), &id);

        already_installed.insert(id.clone());

        match filtered.mods.len() {
            1 => {
                let m: Mod = filtered.mods[0].clone(); // Get the installed mod as Mod instance (read its dependencies)

                // If it does have dependencies
                if let Some(dependencies) = m.dependencies {
                    // Then add the ids to the queue
                    for id in dependencies {
                        if id == "294100" { continue; }
                        if !already_installed.contains(&id) {
                            dependencies_ids.get_mut().insert(id);
                        }
                    }
                }
            }
            x if x > 1 => {
                log!(Error:
                    "Something unexpected happened. Possible duplicated mod: {}",
                    id
                );
                log!(Error: "Filter result: {:?}", filtered);
            }
            _ => {
                log!(Error:
                    "Something unexpected happened with mod {}.",
                    id
                );
                log!(Error: "Probably information files have unexpected names like \"About/about.xml\" instead of \"About/About.xml\"")
            }
        }
    }

    if !ids.is_empty() {
        log!(Warning: "Did not install {:?}", ids);
    }
    
    if d == 0 && args.is_verbose() {
        log!(Status: "Done!");
    };

    //dbg!(&dependencies_ids);

    if args.resolve && !dependencies_ids.get_mut().is_empty() {
        if d == 0 {
            log!( Status:
                "Installing dependencies",
            );
        };
        args.r#mod = Vec::from_iter(dependencies_ids.get_mut().clone());
        install(args.clone(), i.clone(), d + 1, already_installed.clone()).await;
        if d == 0 {
            clear_leftlovers(&path_downloads, &args);
            clear_leftlovers(&PathBuf::from(PATH), &args); 
            log!(Status: "Done!"); 
        };
    }
}
