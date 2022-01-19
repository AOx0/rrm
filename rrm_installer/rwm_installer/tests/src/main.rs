use rwm_installer::*;
use rwm_locals::GamePath;

fn main() {
    let installer = Installer::new(None);
    println!("{:?}", installer)
}
