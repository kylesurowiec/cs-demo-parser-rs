use std::collections::HashMap;

use crate::common::{EquipmentType, map_equipment};
use crate::sendtables1::serverclass::ServerClass;
use crate::stringtables::StringTable;

fn normalize_item_name(name: &str) -> String {
    let mut n = name.to_lowercase();
    if let Some(stripped) = n.strip_prefix("weapon_") {
        n = stripped.to_string();
    }
    n
}

fn equipment_from_table(table: &StringTable) -> Vec<(String, EquipmentType)> {
    table
        .entries
        .values()
        .filter_map(|e| {
            let raw = if !e.value.is_empty() {
                e.value.clone()
            } else if !e.user_data.is_empty() {
                String::from_utf8_lossy(&e.user_data).into_owned()
            } else {
                return None;
            };
            let name = normalize_item_name(&raw);
            let typ = map_equipment(&name);
            if typ == EquipmentType::Unknown {
                None
            } else {
                Some((name, typ))
            }
        })
        .collect()
}

/// Build a mapping from server class names to [`EquipmentType`].
pub fn build_equipment_mapping(
    classes: &[ServerClass],
    item_defs: Option<&StringTable>,
) -> HashMap<String, EquipmentType> {
    let mut out = HashMap::new();

    if let Some(tbl) = item_defs {
        let names = equipment_from_table(tbl);
        if !names.is_empty() {
            for sc in classes {
                let name_lower = sc.name.to_lowercase();
                let dt_lower = sc.data_table_name.to_lowercase();
                for (item, eq) in &names {
                    if name_lower.contains(item) || dt_lower.contains(item) {
                        out.insert(sc.name.clone(), *eq);
                        break;
                    }
                }
            }
            if !out.is_empty() {
                return out;
            }
        }
    }

    // fallback heuristics
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
            let eq = map_equipment(name.trim_start_matches("CWeapon"));
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
