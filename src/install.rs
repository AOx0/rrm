use rrm_scrap::ModSteamInfo;
use crate::utils::*;
use crate::args::Install;
use symlink::symlink_dir;
use crate::{printf};


pub async fn install(args: Install, i: Installer) {
    use rrm_scrap::Filtrable;

    let mut to_install : Vec<ModSteamInfo> = Vec::new();
    let filter_obj = args.to_filter_obj();
    for m in args.r#mod {
        if m.chars().all(char::is_numeric) {
            to_install.push(ModSteamInfo {
                id: m.clone(),
                title: m,
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

            if args.all_yes {
                to_install.push(mods[0].clone());
            } else {
                printf!("Adding {} by {}. Want to continue? [y/n/s(select_other)]: ",&mods[0].title, &mods[0].author);
                n = read();

                if n == "yes" || n == "y" || args.all_yes {
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

    let (_, result) = i.install(&ids);

    let mut successful_ids = Vec::new();

    for line in result.split("\n") {
        if line.contains("Success") {
            let line = line.replace("Success. Downloaded item ", "");
            let words = line.split(" ").next().unwrap();
            successful_ids.push(words.to_string());
        }
    }

    for id in successful_ids {
        #[cfg(target_os="macos")]
            let source = format!("{}", i.config.parent().unwrap().join("Library/Application Support/Steam/steamapps/workshop/content/294100").join(&id).display());

        symlink_dir(source, i.rim_install.as_ref().unwrap().path().join("Mods").join(&id)).unwrap();
    }

    println!("Done! Installed...");
    to_install.iter().for_each(|e|
        println!("    {}", e.title)
    );

}


fn read() -> String {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();

    let n = n.replace("\n", "").replace(" ", "");
    n
}