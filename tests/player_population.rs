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

use cs_demo_parser::parser::EntityEvent;
use cs_demo_parser::sendtables::EntityOp;
use cs_demo_parser::sendtables2::{Class, Entity};

#[test]
fn player_entity_updates() {
    let mut gs = GameState::default();
    let class = Class {
        class_id: 1,
        name: "CCSPlayerPawn".into(),
        serializer: None,
    };
    let ent = Entity {
        index: 2,
        serial: 1,
        class,
    };
    gs.handle_event(&EntityEvent {
        entity: ent.clone(),
        op: EntityOp::CREATED,
    });
    assert!(gs.players_by_entity_id.contains_key(&2));
    let p = gs.players_by_entity_id.get(&2).unwrap();
    assert_eq!(2, p.entity_id);
    assert!(p.is_connected);
    assert!(gs.players_by_user_id.contains_key(&2));
}
