#[test]
pub fn capitalize_stirng_test() {
    assert_eq!("Lowercase string", super::capitalize("lowercase string"))
}

#[test]
pub fn capitalize_long_string_test() {
    assert_eq!(
        "Rust rimworld mod manager".to_string(),
        super::capitalize("rust rimworld mod manager")
    )
}

#[test]
pub fn single_decode_mod_test() {
    use super::ModSteamInfo;

    let target_mod = ModSteamInfo {
        id: 3403180654,
        title: "Alpha Books".to_string(),
        description: "https://i.imgur.com/rLaa7So.png Features Alpha Books brings: - 19 new types of books that are all single-use, and give a bonus to the reader. From useful hediffs, to abilities, to unlocking new map locations, there is variety enough to spice up any playthr...".to_string(),
        author: "".to_string(),
    };

    let script_data = "\n\t\t\t\tSharedFileBindMouseHover( \"sharedfile_3403180654\", false, {\"id\":\"3403180654\",\"title\":\"Alpha Books\",\"description\":\"https:\\/\\/i.imgur.com\\/rLaa7So.png Features Alpha Books brings: - 19 new types of books that are all single-use, and give a bonus to the reader. From useful hediffs, to abilities, to unlocking new map locations, there is variety enough to spice up any playthr...\",\"user_subscribed\":false,\"user_favorited\":false,\"played\":false,\"appid\":294100} );\n\t\t\t";
    let element = super::single_decode_element(script_data.to_string());

    assert_eq!(target_mod, element)
}

#[test]
pub fn gen_headers_test() {
    use super::ModSteamInfo;

    assert_eq!(
        ModSteamInfo::gen_headers(10),
        "       Steam ID   Name         Uploader            \n       --------   --------     --------            ".to_string()
    )
}
