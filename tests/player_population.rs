use cs_demo_parser::game_state::GameState;
use cs_demo_parser::proto::msgs2::CMsgPlayerInfo;
use cs_demo_parser::stringtables::{StringTable, StringTableEntry};
use prost::Message;

#[test]
fn userinfo_updates_players() {
    let mut gs = GameState::default();
    let mut tbl = StringTable {
        name: "userinfo".into(),
        ..Default::default()
    };
    let mut info = CMsgPlayerInfo::default();
    info.name = Some("Alice".into());
    info.userid = Some(1);
    info.xuid = Some(42);
    let mut buf = Vec::new();
    info.encode(&mut buf).unwrap();
    tbl.entries.insert(
        1,
        StringTableEntry {
            value: String::new(),
            user_data: buf,
        },
    );

    gs.apply_userinfo_table(&tbl);

    assert_eq!(1, gs.players_by_user_id.len());
    let p = gs.players_by_user_id.get(&1).unwrap();
    assert_eq!("Alice", p.name);
    assert_eq!(1, p.user_id);
    assert_eq!(42, p.steam_id64);
    assert!(p.is_connected);
}
