use demoinfocs_rs::common::{Equipment, EquipmentType, Player, Team};
use demoinfocs_rs::constants;
use demoinfocs_rs::sendtables::entity::{
    Entity, FlattenedPropEntry, Property, PropertyValue, Vector,
};
use demoinfocs_rs::sendtables::propdecoder::SendTableProperty;
use demoinfocs_rs::sendtables::serverclass::ServerClass;
use std::collections::HashMap;
use std::rc::Rc;

fn make_entity(props: Vec<(&str, i32)>) -> Entity {
    let sc = Rc::new(ServerClass::default());
    let props_vec = props
        .iter()
        .map(|(name, val)| Property {
            entry: FlattenedPropEntry {
                name: name.to_string(),
                prop: SendTableProperty::default(),
                array_element_prop: None,
            },
            value: PropertyValue {
                int_val: *val,
                ..Default::default()
            },
        })
        .collect::<Vec<_>>();
    Entity {
        id: 0,
        serial_num: 0,
        server_class: sc,
        props: props_vec,
    }
}

#[test]
fn ping_score_alive() {
    let ent = make_entity(vec![("m_iPing", 55), ("m_iScore", 3), ("m_iHealth", 10)]);
    let p = Player {
        entity: Some(ent),
        ..Default::default()
    };
    assert_eq!(55, p.ping());
    assert_eq!(3, p.score());
    assert!(p.is_alive());
}

#[test]
fn active_weapon_and_weapons() {
    let mut inv = HashMap::new();
    inv.insert(
        1,
        Equipment {
            equipment_type: EquipmentType::Ak47,
            ..Default::default()
        },
    );
    let ent = make_entity(vec![("m_hActiveWeapon", 1)]);
    let p = Player {
        inventory: inv,
        entity: Some(ent),
        ..Default::default()
    };
    assert!(p.active_weapon().is_some());
    assert_eq!(1, p.weapons().len());
}

#[test]
fn equipment_values_and_gear() {
    let ent = make_entity(vec![
        ("m_unCurrentEquipmentValue", 1600),
        ("m_unRoundStartEquipmentValue", 1500),
        ("m_unFreezetimeEndEquipmentValue", 1400),
        ("m_pItemServices.m_bHasDefuser", 1),
        ("m_pItemServices.m_bHasHelmet", 0),
    ]);
    let p = Player {
        entity: Some(ent),
        ..Default::default()
    };
    assert_eq!(1600, p.equipment_value_current());
    assert_eq!(1500, p.equipment_value_round_start());
    assert_eq!(1400, p.equipment_value_freezetime_end());
    assert!(p.has_defuse_kit());
    assert!(!p.has_helmet());
}

#[test]
fn gear_alt_property_names() {
    let ent = make_entity(vec![("m_bHasDefuser", 1), ("m_bHasHelmet", 1)]);
    let p = Player {
        entity: Some(ent),
        ..Default::default()
    };
    assert!(p.has_defuse_kit());
    assert!(p.has_helmet());
}
