use super::Equipment;
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

impl Default for Team {
    fn default() -> Self {
        Team::Unassigned
    }
}

#[derive(Default)]
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
    pub fn position(&self) -> Vector {
        self.entity
            .as_ref()
            .and_then(|_| Some(self.last_alive_position.clone()))
            .unwrap_or_else(|| self.last_alive_position.clone())
    }
}
