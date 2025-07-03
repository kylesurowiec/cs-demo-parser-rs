use demoinfocs_rs::common::{EquipmentType, map_equipment};
use demoinfocs_rs::parser::datatable::build_equipment_mapping;
use demoinfocs_rs::sendtables::serverclass::ServerClass;

#[test]
fn test_map_equipment_basic() {
    assert_eq!(EquipmentType::Ak47, map_equipment("weapon_ak47"));
    assert_eq!(
        EquipmentType::Knife,
        map_equipment("weapon_knife_butterfly")
    );
    assert_eq!(EquipmentType::Unknown, map_equipment("foobar"));
}

#[test]
fn test_build_equipment_mapping() {
    let sc1 = ServerClass {
        id: 1,
        name: "CC4".into(),
        data_table_id: 0,
        data_table_name: "DT_PlantedC4".into(),
        ..Default::default()
    };
    let sc2 = ServerClass {
        id: 2,
        name: "CWeaponAK47".into(),
        data_table_id: 0,
        data_table_name: "DT_WeaponAK47".into(),
        ..Default::default()
    };
    let sc3 = ServerClass {
        id: 3,
        name: "CSmokeGrenadeProjectile".into(),
        data_table_id: 0,
        data_table_name: "DT_SmokeGrenadeProjectile".into(),
        ..Default::default()
    };
    let map = build_equipment_mapping(&[sc1, sc2, sc3]);
    assert_eq!(Some(&EquipmentType::Bomb), map.get("CC4"));
    assert_eq!(Some(&EquipmentType::Ak47), map.get("CWeaponAK47"));
    assert_eq!(
        Some(&EquipmentType::Smoke),
        map.get("CSmokeGrenadeProjectile")
    );
}
