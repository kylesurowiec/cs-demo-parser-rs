use demoinfocs_rs::utils::{
    convert_steam_id32_to_64, convert_steam_id64_to_32, convert_steam_id_txt_to_32,
};

#[test]
fn convert_txt_to_32() {
    let id = convert_steam_id_txt_to_32("STEAM_0:1:26343269").unwrap();
    assert_eq!(52686539, id);
}

#[test]
fn convert_txt_to_32_error() {
    assert!(convert_steam_id_txt_to_32("STEAM_0:1:a").is_err());
    assert!(convert_steam_id_txt_to_32("STEAM_0:b:21643603").is_err());
    assert!(convert_steam_id_txt_to_32("STEAM_0:b").is_err());
}

#[test]
fn convert_32_to_64() {
    let id = convert_steam_id32_to_64(52686539);
    assert_eq!(76561198012952267u64, id);
}

#[test]
fn convert_64_to_32() {
    let id = convert_steam_id64_to_32(76561198012952267u64);
    assert_eq!(52686539u32, id);
}
