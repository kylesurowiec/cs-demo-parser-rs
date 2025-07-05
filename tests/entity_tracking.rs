use demoinfocs_rs::parser::{EntityEvent, Parser};
use demoinfocs_rs::sendtables::EntityOp;
use demoinfocs_rs::sendtables2::{Class, Entity};
use std::io::Cursor;

#[test]
fn test_projectile_and_dropped_weapon_tracking() {
    let mut p = Parser::new(Cursor::new(Vec::<u8>::new()));

    let proj_class = Class {
        class_id: 1,
        name: "CGrenadeProjectile".into(),
        serializer: None,
    };
    let projectile = Entity {
        index: 1,
        serial: 1,
        class: proj_class,
    };
    p.dispatch_event(EntityEvent {
        entity: projectile.clone(),
        op: EntityOp::CREATED,
    });
    assert!(p.game_state().projectile_owners().contains_key(&1));
    p.dispatch_event(EntityEvent {
        entity: projectile,
        op: EntityOp::DELETED,
    });
    assert!(!p.game_state().projectile_owners().contains_key(&1));

    let drop_class = Class {
        class_id: 2,
        name: "CDroppedWeapon".into(),
        serializer: None,
    };
    let dropped = Entity {
        index: 2,
        serial: 1,
        class: drop_class,
    };
    p.dispatch_event(EntityEvent {
        entity: dropped.clone(),
        op: EntityOp::CREATED,
    });
    assert!(p.game_state().dropped_weapons().contains_key(&2));
    p.dispatch_event(EntityEvent {
        entity: dropped,
        op: EntityOp::DELETED,
    });
    assert!(!p.game_state().dropped_weapons().contains_key(&2));
}
