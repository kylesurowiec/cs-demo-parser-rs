use std::collections::HashMap;

use crate::common::{Bomb, Equipment, GrenadeProjectile, Hostage, Inferno, Player};
use crate::sendtables::entity::Entity;

/// Very small placeholder for a team state.
#[derive(Clone, Default)]
pub struct TeamState {
    pub id: i32,
    pub score: i32,
}

/// Holds all connected participants.
pub struct Participants<'a> {
    players_by_user_id: &'a HashMap<i32, Player>,
    players_by_entity_id: &'a HashMap<i32, Player>,
}

impl<'a> Participants<'a> {
    pub fn by_user_id(&self) -> &HashMap<i32, Player> {
        self.players_by_user_id
    }

    pub fn by_entity_id(&self) -> &HashMap<i32, Player> {
        self.players_by_entity_id
    }
}

#[derive(Clone, Default)]
pub struct GameRules {
    pub con_vars: HashMap<String, String>,
}

/// Representation of the current game state. This is a very small subset of the
/// Go implementation. It only tracks a few basic structures so tests can
/// compile. Further logic will be added as the parser grows.
#[derive(Default)]
pub struct GameState {
    pub ingame_tick: i32,
    pub t_state: TeamState,
    pub ct_state: TeamState,

    pub players_by_user_id: HashMap<i32, Player>,
    pub players_by_entity_id: HashMap<i32, Player>,

    pub grenade_projectiles: HashMap<i32, GrenadeProjectile>,
    pub infernos: HashMap<i32, Inferno>,
    pub weapons: HashMap<i32, Equipment>,
    pub hostages: HashMap<i32, Hostage>,
    pub entities: HashMap<i32, Entity>,
    pub bomb: Bomb,

    pub equipment_mapping: HashMap<String, crate::common::EquipmentType>,

    pub rules: GameRules,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn participants<'a>(&'a self) -> Participants<'a> {
        Participants {
            players_by_user_id: &self.players_by_user_id,
            players_by_entity_id: &self.players_by_entity_id,
        }
    }

    pub fn rules(&self) -> &GameRules {
        &self.rules
    }

    pub fn ingame_tick(&self) -> i32 {
        self.ingame_tick
    }

    pub fn set_ingame_tick(&mut self, tick: i32) {
        self.ingame_tick = tick;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn remove_entity(&mut self, id: i32) {
        self.entities.remove(&id);
    }

    pub fn handle_event<E: 'static>(&mut self, event: &E) {
        let any = event as &dyn std::any::Any;
        if let Some(cv) = any.downcast_ref::<crate::events::ConVarsUpdated>() {
            for (k, v) in &cv.updated_con_vars {
                self.rules.con_vars.insert(k.clone(), v.clone());
            }
        }
    }

    pub fn handle_net_message<M>(&mut self, _msg: &M) {}
}
