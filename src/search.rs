use crate::args::{Local, Steam};
use crate::utils::*;

macro_rules! display_search {
    ($m: expr, $args: expr, $i: expr) => {
        if !$m.is_empty() {
            if $args.display.pager || $i.use_more && !$args.display.no_pager {
                $m.more_display(&$i.with_paging)
            } else {
                $m.display()
            }
        } else {
            println!("No results found")
        }
    };
}

pub fn search_locally(i: Installer, args: Local) {
    use rrm_locals::Filtrable;

    let d_type = rrm_locals::DisplayType::from(args.display.large);
    let mods = GameMods::from(i.rim_install.unwrap()).with_display(d_type);

    let filtered = mods.filter_by(args.to_filter_obj(), &args.string);

    display_search!(filtered, args, i);
}

pub async fn search_steam(i: Installer, args: Steam) {
    use rrm_scrap::Filtrable;

    let mods = SteamMods::search(&args.r#mod)
        .await
        .with_display(rrm_locals::DisplayType::from(args.display.large));

    let mods = if args.filter.is_some() {
        let value = if args.filter.as_ref().unwrap().is_some() {
            args.filter.as_ref().unwrap().clone().unwrap()
        } else {
            args.r#mod.clone()
        };

        mods.filter_by(args.to_filter_obj(), &value)
    } else {
        mods
    };

    display_search!(mods, args, i);
}