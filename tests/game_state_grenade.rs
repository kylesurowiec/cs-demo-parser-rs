use demoinfocs_rs::parser::{EntityEvent, Parser};
use demoinfocs_rs::sendtables::EntityOp;
use demoinfocs_rs::sendtables2::{Class, Entity};
use std::io::Cursor;

#[test]
fn track_active_grenades_and_infernos() {
    let mut parser = Parser::new(Cursor::new(Vec::<u8>::new()));

    let g_class = Class {
        class_id: 1,
        name: "CSmokeGrenadeProjectile".into(),
        serializer: None,
    };
    let grenade = Entity {
        index: 1,
        serial: 1,
        class: g_class,
    };
    parser.dispatch_event(EntityEvent {
        entity: grenade.clone(),
        op: EntityOp::CREATED,
    });
    assert_eq!(1, parser.game_state().grenade_projectiles().len());

    parser.dispatch_event(EntityEvent {
        entity: grenade,
        op: EntityOp::DELETED,
    });
    assert_eq!(0, parser.game_state().grenade_projectiles().len());

    let i_class = Class {
        class_id: 2,
        name: "CInferno".into(),
        serializer: None,
    };
    let inferno = Entity {
        index: 2,
        serial: 1,
        class: i_class,
    };
    parser.dispatch_event(EntityEvent {
        entity: inferno.clone(),
        op: EntityOp::CREATED,
    });
    assert_eq!(1, parser.game_state().infernos().len());

    parser.dispatch_event(EntityEvent {
        entity: inferno,
        op: EntityOp::DELETED,
    });
    assert_eq!(0, parser.game_state().infernos().len());
}

#[test]
fn track_grenade_trajectory() {
    use demoinfocs_rs::common::new_grenade_projectile;
    use demoinfocs_rs::sendtables::entity::Vector;
    use std::time::Duration;

    let mut gs = demoinfocs_rs::game_state::GameState::default();
    let g = new_grenade_projectile();
    gs.grenade_projectiles.insert(1, g);

    gs.track_grenade_position(
        1,
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        1,
        Duration::from_millis(1),
    );
    gs.track_grenade_position(
        1,
        Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        2,
        Duration::from_millis(2),
    );

    let tracked = gs.grenade_projectiles.get(&1).unwrap();
    assert_eq!(2, tracked.trajectory.len());
    assert_eq!(tracked.trajectory2[1].frame_id, 2);
}
