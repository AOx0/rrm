use crate::args::InstallingOptions;
use tokio::{io::{BufReader, AsyncBufReadExt}, process::Command};
use std::process::Stdio;
use rrm_installer::Installer;
use crate::utils::*;
use async_recursion::async_recursion;
use std::sync::{Arc, Mutex};

#[async_recursion(?Send)]
pub async fn install<T: InstallingOptions>(args: T, mods: &[&str], installer: Installer, start_file_watcher: Arc<Mutex<bool>>) -> String {
    let install_message = Installer::gen_install_string(&mods);
    let steamcmd = installer.get_steamcmd_path();

    log!( Status: "Spawning SteamCMD");
   

    #[cfg(target_os = "windows")]
    let mut cmd = {
        if args.is_debug() {
            log!(Status: "Spawning with command \"{} {}\"", steamcmd.as_path().to_str().unwrap(), "+login anonymous {} +quit"
                    .replace("{}", &install_message)
            );
        }
        let mut cmd = Command::new(steamcmd.as_path().to_str().unwrap());
        cmd.args(
            "+login anonymous {} +quit"
                .replace("{}", &install_message)
                .split(" "),
        );
        cmd.stdout(Stdio::piped());
        cmd
    }; 
 
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let mut cmd = {
        if args.is_debug() {
            log!(Status: "Spawning with command \"{} {}\"", "env", 
                    r#"HOME=PATH [] +login anonymous {} +quit"#
                    .replace(
                        "PATH",
                        installer
                            .config
                            .parent()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap(),
                    )
                    .replace("[]", steamcmd.as_path().to_str().unwrap())
                    .replace("{}", &install_message)
            );
        }
        let mut cmd = Command::new("env");
        cmd.args(
            r#"HOME=PATH [] +login anonymous {} +quit"#
                .replace(
                    "PATH",
                    installer
                        .config
                        .parent()
                        .unwrap()
                        .as_os_str()
                        .to_str()
                        .unwrap(),
                )
                .replace("[]", steamcmd.as_path().to_str().unwrap())
                .replace("{}", &install_message)
                .split(' '),
        );
        cmd.stdout(Stdio::piped());
        cmd
    };

    let mut child = cmd.spawn().unwrap();

    if args.is_verbose() { log!( Status: "Done spawning"); }

    {
        let mut start = start_file_watcher.lock().unwrap();
        *start = true;
    }

    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    let mut overall_result = String::new();

    let mut did_update = false;

    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("Update complete") {
            log!(Warning: "SteamCMD updated");
            did_update = true;
        }

        overall_result += &(line.clone() + "\n");
        if args.is_debug() {
            log!(Received: "{line}");
        } else if args.is_debug() && line.contains("Success") {
            log!(Warning: 
                "{})",
                &line[0..line.find(") ").unwrap_or_else(|| line.len())]
            );
        }

    } 


    if did_update {
        log!(Warning: "Retrying installation beacause SteamCMD update canceled it");
        install(args, mods, installer, start_file_watcher).await
    } else {
        overall_result
    }
}


