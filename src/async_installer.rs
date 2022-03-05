use crate::args::InstallingOptions;
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use rrm_installer::Installer;

use crate::printf;
use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn install<T: InstallingOptions>(args: T, mods: &[&str], installer: Installer) -> String {
    let install_message = Installer::gen_install_string(&mods);
    let steamcmd = installer.get_steamcmd_path();

    #[cfg(target_os = "windows")]
    let mut child = Command::new(steamcmd.as_path().to_str().unwrap())
        .args(
            "+login anonymous {} +quit"
                .replace("{}", &install_message)
                .split(" "),
        )
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if args.is_debug() {
        println!("{}", install_message);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let mut child = Command::new("env")
        .args(
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
        )
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    let mut overall_result = String::new();

    let mut did_update = false;

    while let Some(Ok(line)) = lines.next().await {
        if line.contains("Update complete") {
            did_update = true;
        }

        overall_result += &(line.clone() + "\n");
        if line.contains("Success") && args.is_verbose() {
            printf!(
                "{})\n",
                &line[0..line.find(") ").unwrap_or_else(|| line.len())]
            );
        } else if args.is_debug() {
            printf!("{}\n", &line);
        }
    }

    if did_update {
        install(args, mods, installer).await
    } else {
        overall_result
    }
}
