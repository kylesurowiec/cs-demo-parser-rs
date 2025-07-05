use std::collections::HashMap;

use crate::sendtables2::Entity;

#[derive(Clone, Default)]
pub struct GameRules {
    pub con_vars: HashMap<String, String>,
    pub entity: Option<Entity>,
}

impl GameRules {
    pub fn update_con_vars(&mut self, vars: &HashMap<String, String>) {
        for (k, v) in vars {
            self.con_vars.insert(k.clone(), v.clone());
        }
    }

    pub fn round_time(&self) -> Option<std::time::Duration> {
        self.con_vars
            .get("mp_roundtime")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }

    pub fn freeze_time(&self) -> Option<std::time::Duration> {
        self.con_vars
            .get("mp_freezetime")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }

    pub fn bomb_time(&self) -> Option<std::time::Duration> {
        self.con_vars
            .get("mp_c4timer")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }
}
