use crate::args::{Local, Steam};
use crate::utils::*;
use rwm_installer::Installer;
use rwm_locals::Filtrable;

pub fn search_locally(i: Installer, args: Local) {
    let d_type = rwm_locals::DisplayType::from(args.display.large);
    let mods = GameMods::from(i.rim_install.unwrap()).with_display(d_type);

    let filtered = mods.filter_by(args.to_filter_obj(), &args.string);

    if !filtered.is_empty() {
        if args.display.pager || i.use_more && !args.display.no_pager {
            filtered.more_display(&i.with_paging)
        } else {
            filtered.display()
        }
    } else {
        println!("No results found")
    }
}

pub async fn search_steam(args: Steam) {
    let mods = SteamMods::search(&args.r#mod)
        .await
        .with_display(rwm_locals::DisplayType::from(args.display.large));

    mods.display();
}
