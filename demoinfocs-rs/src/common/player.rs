use super::{Equipment, EquipmentType};
use crate::constants;
use crate::sendtables::entity::{Entity, Vector};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Team {
    Unassigned = 0,
    Spectators = 1,
    Terrorists = 2,
    CounterTerrorists = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Color {
    Grey = -1,
    Yellow = 0,
    Purple = 1,
    Green = 2,
    Blue = 3,
    Orange = 4,
}

impl Default for Color {
    fn default() -> Self {
        Color::Grey
    }
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub steam_id64: u64,
    pub last_alive_position: Vector,
    pub user_id: i32,
    pub name: String,
    pub inventory: HashMap<i32, Equipment>,
    pub ammo_left: [i32; 32],
    pub entity_id: i32,
    pub entity: Option<Entity>,
    pub flash_duration: f32,
    pub flash_tick: i32,
    pub team: Team,
    pub is_bot: bool,
    pub is_connected: bool,
    pub is_defusing: bool,
    pub is_planting: bool,
    pub is_reloading: bool,
    pub is_unknown: bool,
    pub previous_frame_position: Vector,
}

impl Player {
    pub fn ping(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_iPing"))
            .map(|v| {
                if v.int_val != 0 {
                    v.int_val
                } else {
                    v.int64_val as i32
                }
            })
            .unwrap_or(0)
    }

    pub fn score(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_iScore"))
            .map(|v| v.int_val)
            .unwrap_or(0)
    }

    pub fn health(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_iHealth"))
            .map(|v| v.int_val)
            .unwrap_or(0)
    }

    pub fn is_alive(&self) -> bool {
        if self.health() > 0 {
            return true;
        }
        if let Some(ent) = &self.entity {
            if let Some(v) = ent.property_value("m_lifeState") {
                return v.int_val == 0;
            }
            if let Some(v) = ent.property_value("m_bPawnIsAlive") {
                return v.bool_val();
            }
        }
        false
    }

    fn active_weapon_id(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_hActiveWeapon"))
            .map(|v| v.int_val & constants::ENTITY_HANDLE_INDEX_MASK as i32)
            .unwrap_or(0)
    }

    pub fn active_weapon(&self) -> Option<&Equipment> {
        let id = self.active_weapon_id();
        self.inventory.get(&id)
    }

    pub fn weapons(&self) -> Vec<&Equipment> {
        self.inventory.values().collect()
    }

    pub fn equipment_value_current(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_unCurrentEquipmentValue"))
            .map(|v| v.int_val)
            .unwrap_or(0)
    }

    pub fn equipment_value_round_start(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_unRoundStartEquipmentValue"))
            .map(|v| v.int_val)
            .unwrap_or(0)
    }

    pub fn equipment_value_freezetime_end(&self) -> i32 {
        self.entity
            .as_ref()
            .and_then(|e| e.property_value("m_unFreezetimeEndEquipmentValue"))
            .map(|v| v.int_val)
            .unwrap_or(0)
    }

    pub fn has_defuse_kit(&self) -> bool {
        self.entity
            .as_ref()
            .and_then(|e| {
                e.property_value("m_pItemServices.m_bHasDefuser")
                    .or_else(|| e.property_value("m_bHasDefuser"))
            })
            .map(|v| v.bool_val())
            .unwrap_or(false)
    }

    pub fn has_helmet(&self) -> bool {
        self.entity
            .as_ref()
            .and_then(|e| {
                e.property_value("m_pItemServices.m_bHasHelmet")
                    .or_else(|| e.property_value("m_bHasHelmet"))
            })
            .map(|v| v.bool_val())
            .unwrap_or(false)
    }
}
