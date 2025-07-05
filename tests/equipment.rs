use demoinfocs_rs::common::{EquipmentType, map_equipment};
use demoinfocs_rs::parser::datatable::build_equipment_mapping;
use demoinfocs_rs::sendtables::serverclass::ServerClass;
use demoinfocs_rs::stringtables::{StringTable, StringTableEntry};

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
    let map = build_equipment_mapping(&[sc1, sc2, sc3], None);
    assert_eq!(Some(&EquipmentType::Bomb), map.get("CC4"));
    assert_eq!(Some(&EquipmentType::Ak47), map.get("CWeaponAK47"));
    assert_eq!(
        Some(&EquipmentType::Smoke),
        map.get("CSmokeGrenadeProjectile")
    );
}

#[test]
fn test_build_equipment_mapping_itemdefs() {
    let sc1 = ServerClass {
        id: 1,
        name: "CWeaponAK47".into(),
        data_table_id: 0,
        data_table_name: "DT_WeaponAK47".into(),
        ..Default::default()
    };
    let sc2 = ServerClass {
        id: 2,
        name: "CSmokeGrenadeProjectile".into(),
        data_table_id: 0,
        data_table_name: "DT_SmokeGrenadeProjectile".into(),
        ..Default::default()
    };

    let mut tbl = StringTable {
        name: "ItemDefinitions".into(),
        ..Default::default()
    };
    tbl.entries.insert(
        0,
        StringTableEntry {
            value: "weapon_ak47".into(),
            user_data: Vec::new(),
        },
    );
    tbl.entries.insert(
        1,
        StringTableEntry {
            value: "smokegrenade".into(),
            user_data: Vec::new(),
        },
    );

    let map = build_equipment_mapping(&[sc1.clone(), sc2.clone()], Some(&tbl));

    assert_eq!(Some(&EquipmentType::Ak47), map.get(&sc1.name));
    assert_eq!(Some(&EquipmentType::Smoke), map.get(&sc2.name));
}
