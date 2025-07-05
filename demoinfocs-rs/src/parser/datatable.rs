use std::collections::HashMap;

use crate::common::{EquipmentType, map_equipment};
use crate::sendtables1::serverclass::ServerClass;

/// Build a mapping from server class names to [`EquipmentType`].
pub fn build_equipment_mapping(classes: &[ServerClass]) -> HashMap<String, EquipmentType> {
    let mut out = HashMap::new();
    for sc in classes {
        let name = sc.name.clone();
        if name == "CC4" {
            out.insert(name, EquipmentType::Bomb);
            continue;
        }
        if name == "CKnife" {
            out.insert(name, EquipmentType::Knife);
            continue;
        }
        if name.starts_with("CWeapon") {
            let eq = map_equipment(&name[7..]);
            out.insert(name, eq);
            continue;
        }
        let dt = sc.data_table_name.as_str();
        if dt.starts_with("DT_Weapon") {
            let eq = map_equipment(&dt[9..]);
            out.insert(name, eq);
        } else if dt.starts_with("DT_") {
            let eq = map_equipment(&dt[3..]);
            if eq != EquipmentType::Unknown {
                out.insert(name, eq);
            }
        }
    }
    out
}
