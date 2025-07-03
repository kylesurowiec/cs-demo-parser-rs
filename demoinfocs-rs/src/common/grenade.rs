use super::{Equipment, Player};
use crate::sendtables::entity::{Entity, Vector};
use std::time::Duration;

#[derive(Default)]
pub struct TrajectoryEntry {
    pub position: Vector,
    pub frame_id: i32,
    pub time: Duration,
}

#[derive(Default)]
pub struct GrenadeProjectile {
    pub entity: Option<Entity>,
    pub weapon_instance: Option<Equipment>,
    pub thrower: Option<Player>,
    pub owner: Option<Player>,
    pub trajectory: Vec<Vector>,
    pub trajectory2: Vec<TrajectoryEntry>,
    pub unique_id: i64,
}

use std::sync::atomic::{AtomicI64, Ordering};

static NEXT_ID: AtomicI64 = AtomicI64::new(1);

pub fn new_grenade_projectile() -> GrenadeProjectile {
    GrenadeProjectile {
        unique_id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        ..Default::default()
    }
}
