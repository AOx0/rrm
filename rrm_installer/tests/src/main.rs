use rrm_installer::*;
use rrm_locals::GamePath;

fn main() {
    let installer = Installer::new(None);
    println!("{:?}", installer)
}
