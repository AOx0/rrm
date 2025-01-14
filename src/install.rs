use crate::args::{InstallCommandGroup, InstallingOptions};
use crate::printf;
use crate::utils::*;
use async_recursion::async_recursion;
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use notify::Event;
use notify::Watcher;
use notify::event::CreateKind;
use regex::Regex;
use rrm_locals::{FilterBy, Filtrable};
use rrm_scrap::{FlagSet, ModSteamInfo};
use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use text_io::try_read;

#[cfg(target_os = "windows")]
const PATH: &str = r"steamcmd\steamapps\workshop\content\294100\";

#[cfg(target_os = "linux")]
const PATH: &str = r"Steam/steamapps/workshop/content/294100/";

#[cfg(target_os = "macos")]
const PATH: &str = r"Library/Application Support/Steam/steamapps/workshop/content/294100";

#[cfg(target_os = "linux")]
pub const TMP_PATH: &str = "/tmp/rrm-downloads";

fn download_cleanup() {
    let tmp_path = PathBuf::from(TMP_PATH);

    if !tmp_path.exists() {};

    std::fs::remove_dir_all(tmp_path).unwrap();
}

#[async_recursion(?Send)]
pub async fn install(
    mut args: InstallCommandGroup,
    i: Installer,
    d: usize,
    mut already_installed: HashSet<usize>,
) {
    if args.is_debug() {
        log!(Warning: "Already installed {:?}", already_installed);
    }
    let inline: bool = !(args.r#mod.len() == 1 && args.r#mod.first().unwrap() == "None");
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

    for mod_identifier in &args.r#mod {
        if mod_identifier.chars().all(char::is_numeric) {
            if mod_identifier.trim().is_empty() {
                continue;
            }

            if args.is_verbose() {
                log!(Status: "Adding {} to queue", mod_identifier);
            }

            to_install.push(ModSteamInfo {
                id: mod_identifier.parse().unwrap(),
                title: mod_identifier.clone(),
                description: "".to_string(),
                author: "".to_string(),
            });
            continue;
        }

        if mod_identifier.contains("steamcommunity.com") {
            let m = mod_identifier.replace(['\n', ' '], "");

            if let Some(id) = extract_id(&m, &re) {
                if id.trim().is_empty() {
                    continue;
                }

                if args.is_verbose() {
                    log!(Status: "Adding {} to queue", id);
                }

                to_install.push(ModSteamInfo {
                    id: 0,
                    title: id.clone(),
                    description: "".to_string(),
                    author: "".to_string(),
                });
                continue;
            }
        }

        let mods = SteamMods::search(mod_identifier)
            .await
            .with_raw_display(None);

        let mut mods = if args.filter.is_some() {
            let value = if args.filter.as_ref().unwrap().is_some() {
                args.filter.as_ref().unwrap().clone().unwrap()
            } else {
                mod_identifier.clone()
            };

            mods.filter_by(filter_obj, &value)
        } else {
            mods
        };

        if mods.is_empty() {
            if !inline || args.is_verbose() {
                log!( Error: "No results found for {}", mod_identifier);
            }
            continue;
        }

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
            std::process::exit(0)
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
    }

    if d == 0 {
        log!( Status:
            "Installing mod{}",
            if args.r#mod.len() > 1 { "s" } else { "" },
        )
    };

    let path_downloads = PathBuf::from([TMP_PATH, PATH].join("/"));
    log!(Info: "download path is: {}", &path_downloads.display());

    if args.is_verbose() {
        log!(Warning: "Starting file watcher");
    }

    let mut cur = 0;
    let mut last_printed = String::new();
    let to_install_closure = to_install.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
        Ok(event) => {
            if let notify::EventKind::Create(CreateKind::Folder) = event.kind {
                let path = event.paths[0].to_owned();
                let current = path.file_name().unwrap().to_str().unwrap().to_owned();
                if let Some(n) = path.parent() {
                    if let Some(name) = n.file_name() {
                        let name = name.to_str().unwrap();
                        if name == "294100" && current != last_printed {
                            cur += 1;
                            log!(Status: "[{1:0>3}/{2:0>3}] Downloading {0}", current, cur, &to_install_closure.len());
                            last_printed = current;
                        }
                    }
                }
            }
        }
        Err(e) => log!(Error: "error event: {:?}", e),
    }).unwrap();

    let result = {
        let mut result = String::new();
        let mut num = to_install.len();
        let mut n = 0;
        let large_list = num > 200;

        if large_list {
            log!(Warning: "More than 200 mods. Splitting to chunks of 200 to avoid problems with SteamCMD");
        }

        while num >= 200 {
            log!(Warning: "Spawning SteamCMD with mods {}..{}", n, n+200);
            let r = crate::async_installer::install(
                args.clone(),
                to_install.get(n..n + 200).unwrap().to_vec(),
                i.clone(),
                &mut watcher,
                &path_downloads,
            )
            .await;
            result.push_str(&r);
            num = 200;
            n += 200;
        }

        if large_list {
            log!(Warning: "Spawning last SteamCMD with mods {}..{}", n, n+num);
        }
        let r = crate::async_installer::install(
            args.clone(),
            to_install.get(n..n + num).unwrap().to_vec(),
            i.clone(),
            &mut watcher,
            &path_downloads,
        )
        .await;
        result.push_str(&r);
        result
    };

    if args.is_verbose() {
        log!(Status: "Installer finished");
    }

    watcher
        .unwatch(&rrm_installer::get_or_create_config_dir().join(path_downloads.parent().unwrap()))
        .unwrap();

    let mut successful_ids: HashSet<usize> = HashSet::new();

    for line in result.split('\n') {
        if line.contains("Success") {
            let line = line.replace("Success. Downloaded item ", "");
            let words = line.split(' ').next().unwrap();
            successful_ids.insert(words.to_string().parse().unwrap());
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

    for id in successful_ids {
        let id_download_path: PathBuf = PathBuf::from([TMP_PATH, PATH, &id.to_string()].join("/"));
        let options = CopyOptions {
            overwrite: true,
            ..Default::default()
        };

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), id);

        let mut ignored = false;

        for old_mod in filtered.mods {
            //dbg!(&old_mod.path);
            if old_mod.path.starts_with('_') {
                if args.is_verbose() {
                    log!( Warning: "Ignoring {}", old_mod.path);
                }
                ignored = true;
                dir::remove(&id_download_path).unwrap();
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
                id_download_path.display().to_string(),
                destination.to_str().unwrap_or("error")
            );
        }

        dir::move_dir(&id_download_path, &destination, &options).unwrap();

        let installed_mods =
            GameMods::from(rim_install.path().to_str().unwrap()).with_display(DisplayType::Short);
        let filtered = installed_mods.filter_by(FlagSet::from(FilterBy::SteamID), id);

        already_installed.insert(id);

        match filtered.mods.len() {
            1 => {
                let m: Mod = filtered.mods[0].clone(); // Get the installed mod as Mod instance (read its dependencies)

                // If it does have dependencies
                if let Some(dependencies) = m.dependencies {
                    // Then add the ids to the queue
                    let deps = dependencies.iter().map(|dep_id| dep_id.parse().unwrap());
                    for id in deps {
                        if id == 294100 {
                            continue;
                        }
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

    // TODO: Update this log
    // if !ids.is_empty() {
    //     log!(Warning: "Did not install {:?}", ids);
    // }

    if d == 0 && args.is_verbose() {
        log!(Status: "Done!");
    };

    //dbg!(&dependencies_ids);
    //

    log!(Info: "Cleaning up temporary folders...");
    download_cleanup();

    if args.resolve && !dependencies_ids.get_mut().is_empty() {
        if d == 0 {
            log!( Status:
                "Installing dependencies",
            );
        };
        args.r#mod = Vec::from_iter(
            dependencies_ids
                .get_mut()
                .iter()
                .map(|id| id.to_string())
                .clone(),
        );
        install(args.clone(), i.clone(), d + 1, already_installed.clone()).await;
        if d == 0 {
            log!(Status: "Done!");
        };
    }
}
