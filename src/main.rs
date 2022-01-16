use rwm_locals::DisplayType;

mod statics;
mod args;
mod list;

#[tokio::main]
async fn main() {
    let args = args::Args::load();

    if let args::Commands::List { game_path, large } = args.command {
        list::list(&game_path, DisplayType::from(large))
    }
}