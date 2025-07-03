use super::{Equipment, Player};
use crate::sendtables::entity::{Entity, Vector};
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct TrajectoryEntry {
    pub position: Vector,
    pub frame_id: i32,
    pub time: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct GrenadeProjectile {
    pub entity: Option<Entity>,
    pub weapon_instance: Option<Equipment>,
    pub thrower: Option<Player>,
    pub owner: Option<Player>,
    pub trajectory: Vec<Vector>,
    pub trajectory2: Vec<TrajectoryEntry>,
    pub unique_id: i64,
}
