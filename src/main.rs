use std::path::Path;
use rwm_list::*;

mod statics;
mod args;

#[tokio::main]
async fn main() {
    let args = args::Args::load();
    println!("{:?}", args);
    println!("{:?}", statics::RW_DEFAULT_PATH);

    let p: &Path = Path::new("./path/to/123/456");
    eprintln!("{}", expand!(p).display());

    if let args::Commands::Install { r#mod } = args.command {
        println!("Su nombre es {}", r#mod);
    }
}