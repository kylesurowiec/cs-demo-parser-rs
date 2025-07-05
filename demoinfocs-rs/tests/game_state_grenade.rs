use demoinfocs_rs::parser::{Parser, EntityEvent};
use demoinfocs_rs::sendtables::EntityOp;
use demoinfocs_rs::sendtables2::{Class, Entity};
use std::io::Cursor;

#[test]
fn track_active_grenades_and_infernos() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let g_class = Class { class_id: 1, name: "CSmokeGrenadeProjectile".into(), serializer: None };
    let grenade = Entity { index: 1, serial: 1, class: g_class };
    parser.dispatch_event(EntityEvent { entity: grenade.clone(), op: EntityOp::CREATED });
    assert_eq!(1, parser.game_state().grenade_projectiles().len());

    parser.dispatch_event(EntityEvent { entity: grenade, op: EntityOp::DELETED });
    assert_eq!(0, parser.game_state().grenade_projectiles().len());

    let i_class = Class { class_id: 2, name: "CInferno".into(), serializer: None };
    let inferno = Entity { index: 2, serial: 1, class: i_class };
    parser.dispatch_event(EntityEvent { entity: inferno.clone(), op: EntityOp::CREATED });
    assert_eq!(1, parser.game_state().infernos().len());

    parser.dispatch_event(EntityEvent { entity: inferno, op: EntityOp::DELETED });
    assert_eq!(0, parser.game_state().infernos().len());
}
