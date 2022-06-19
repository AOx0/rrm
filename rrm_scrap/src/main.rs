use rrm_locals::DisplayType;
use rrm_scrap::SteamMods;

#[tokio::main]
async fn main() {
    let mods = SteamMods::search("Fluffy")
        .await
        .with_display(DisplayType::Long);

    mods.display();

    let mods = mods.with_display(DisplayType::Short);

    mods.display();
}
