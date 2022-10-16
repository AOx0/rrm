use crate::args::InstallingOptions;
use crate::utils::*;
use async_recursion::async_recursion;
use notify::{RecommendedWatcher, Watcher};
use rrm_installer::{get_or_create_config_dir, Installer};
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[async_recursion(?Send)]
pub async fn install<T: InstallingOptions>(
    args: T,
    mods: &[&str],
    installer: Installer,
    start_file_watcher: &mut RecommendedWatcher,
    path_downloads: &Path,
) -> String {
    let install_message = Installer::gen_install_string(&mods);
    let steamcmd = installer.get_steamcmd_path();

    if args.is_verbose() {
        log!( Status: "Spawning SteamCMD");
    }

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
                        rrm_installer::get_or_create_config_dir()
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
                    rrm_installer::get_or_create_config_dir()
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

    if args.is_verbose() {
        log!( Status: "Done spawning");
    }

    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    let mut overall_result = String::new();

    let mut did_update = false;

    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("Waiting for client config...OK") {

            if !&get_or_create_config_dir().join(path_downloads.parent().unwrap()).exists() {
                std::fs::create_dir_all(&get_or_create_config_dir().join(path_downloads.parent().unwrap())).unwrap();
            }

            start_file_watcher
                .watch(
                    &get_or_create_config_dir().join(path_downloads.parent().unwrap()),
                    notify::RecursiveMode::Recursive,
                )
                .unwrap();
        }
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
        log!(Warning: "Retrying installation because SteamCMD update canceled it");
        install(args, mods, installer, start_file_watcher, path_downloads).await
    } else {
        overall_result
    }
}
