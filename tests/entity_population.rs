use cs_demo_parser::common::{EquipmentType, Player};
use cs_demo_parser::game_state::GameState;
use cs_demo_parser::parser::EntityEvent;
use cs_demo_parser::sendtables::EntityOp;
use cs_demo_parser::sendtables2::{Class, Entity};

#[test]
fn weapon_entities_are_tracked() {
    let mut gs = GameState::default();
    gs.equipment_mapping
        .insert("CWeaponAK47".into(), EquipmentType::Ak47);
    let class = Class {
        class_id: 1,
        name: "CWeaponAK47".into(),
        serializer: None,
    };
    let ent = Entity {
        index: 10,
        serial: 1,
        class,
    };

    gs.handle_event(&EntityEvent {
        entity: ent.clone(),
        op: EntityOp::CREATED,
    });
    assert_eq!(1, gs.weapons.len());
    assert_eq!(
        EquipmentType::Ak47,
        gs.weapons.get(&10).unwrap().equipment_type
    );

    gs.handle_event(&EntityEvent {
        entity: ent,
        op: EntityOp::DELETED,
    });
    assert!(gs.weapons.is_empty());
}
