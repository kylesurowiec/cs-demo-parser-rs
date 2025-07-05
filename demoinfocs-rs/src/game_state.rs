use std::any::Any;
use std::collections::HashMap;

use crate::common::{Bomb, Equipment, GrenadeProjectile, Hostage, Inferno, Player, Team};
use crate::proto::msg::cs_demo_parser_rs as proto_msg;
use crate::sendtables2::Entity;

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

    pub fn all(&self) -> Vec<&Player> {
        self.players_by_user_id.values().collect()
    }

    pub fn connected(&self) -> Vec<&Player> {
        self.players_by_user_id
            .values()
            .filter(|p| p.is_connected)
            .collect()
    }

    pub fn team_members(&self, team: Team) -> Vec<&Player> {
        self.players_by_user_id
            .values()
            .filter(|p| p.team == team)
            .collect()
    }
}

#[derive(Clone, Default)]
pub struct GameRules {
    pub con_vars: HashMap<String, String>,
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct LastFlash {
    pub player: Option<Player>,
    pub projectile_by_player: HashMap<i32, GrenadeProjectile>,
}

#[derive(Default)]
pub struct FlyingFlashbang {
    pub projectile: Option<GrenadeProjectile>,
    pub flashed_entity_ids: Vec<i32>,
    pub exploded_frame: i32,
}

/// Representation of the current game state. This is a very small subset of the
/// Go implementation. It only tracks a few basic structures so tests can
/// compile. Further logic will be added as the parser grows.
#[derive(Default)]
pub struct GameState {
    pub ingame_tick: i32,
    pub t_state: TeamState,
    pub ct_state: TeamState,

    pub total_rounds_played: i32,
    pub game_phase: crate::events::GamePhase,
    pub is_warmup_period: bool,
    pub is_freezetime: bool,
    pub is_match_started: bool,
    pub overtime_count: i32,

    pub players_by_user_id: HashMap<i32, Player>,
    pub players_by_entity_id: HashMap<i32, Player>,

    pub grenade_projectiles: HashMap<i32, GrenadeProjectile>,
    pub infernos: HashMap<i32, Inferno>,
    pub weapons: HashMap<i32, Equipment>,
    pub hostages: HashMap<i32, Hostage>,
    pub entities: HashMap<i32, Entity>,
    pub projectile_owners: HashMap<i32, i32>,
    pub dropped_weapons: HashMap<i32, String>,
    pub bomb: Bomb,

    pub current_defuser: Option<Player>,
    pub current_planter: Option<Player>,

    pub thrown_grenades: HashMap<i32, Vec<Equipment>>,
    pub flying_flashbangs: Vec<FlyingFlashbang>,
    pub last_flash: LastFlash,

    pub equipment_mapping: HashMap<String, crate::common::EquipmentType>,

    pub rules: GameRules,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team(&self, team: Team) -> Option<&TeamState> {
        match team {
            | Team::Terrorists => Some(&self.t_state),
            | Team::CounterTerrorists => Some(&self.ct_state),
            | _ => None,
        }
    }

    pub fn team_counter_terrorists(&self) -> &TeamState {
        &self.ct_state
    }

    pub fn team_terrorists(&self) -> &TeamState {
        &self.t_state
    }

    pub fn participants<'a>(&'a self) -> Participants<'a> {
        Participants {
            players_by_user_id: &self.players_by_user_id,
            players_by_entity_id: &self.players_by_entity_id,
        }
    }

    pub fn grenade_projectiles(&self) -> &HashMap<i32, GrenadeProjectile> {
        &self.grenade_projectiles
    }

    pub fn infernos(&self) -> &HashMap<i32, Inferno> {
        &self.infernos
    }

    pub fn weapons(&self) -> &HashMap<i32, Equipment> {
        &self.weapons
    }

    pub fn projectile_owners(&self) -> &HashMap<i32, i32> {
        &self.projectile_owners
    }

    pub fn dropped_weapons(&self) -> &HashMap<i32, String> {
        &self.dropped_weapons
    }

    pub fn entities(&self) -> &HashMap<i32, Entity> {
        &self.entities
    }

    pub fn bomb(&self) -> &Bomb {
        &self.bomb
    }

    pub fn total_rounds_played(&self) -> i32 {
        self.total_rounds_played
    }

    pub fn game_phase(&self) -> crate::events::GamePhase {
        self.game_phase
    }

    pub fn is_warmup_period(&self) -> bool {
        self.is_warmup_period
    }

    pub fn is_freezetime_period(&self) -> bool {
        self.is_freezetime
    }

    pub fn is_match_started(&self) -> bool {
        self.is_match_started
    }

    pub fn overtime_count(&self) -> i32 {
        self.overtime_count
    }

    pub fn rules(&self) -> &GameRules {
        &self.rules
    }

    pub fn match_settings(&self) -> &std::collections::HashMap<String, String> {
        &self.rules.con_vars
    }

    pub fn round_time(&self) -> Option<std::time::Duration> {
        self.rules
            .con_vars
            .get("mp_roundtime")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }

    pub fn freeze_time(&self) -> Option<std::time::Duration> {
        self.rules
            .con_vars
            .get("mp_freezetime")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }

    pub fn bomb_time(&self) -> Option<std::time::Duration> {
        self.rules
            .con_vars
            .get("mp_c4timer")
            .and_then(|v| v.parse::<u64>().ok())
            .map(std::time::Duration::from_secs)
    }

    pub fn ingame_tick(&self) -> i32 {
        self.ingame_tick
    }

    pub fn set_ingame_tick(&mut self, tick: i32) {
        self.ingame_tick = tick;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.index, entity);
    }

    pub fn remove_entity(&mut self, id: i32) {
        self.entities.remove(&id);
    }

    fn update_special_entities(&mut self, ent: &Entity) {
        let name = ent.class.name.as_str();
        if name.contains("Projectile") {
            self.projectile_owners.entry(ent.index).or_insert(0);
        } else if name.contains("DroppedWeapon") || name.contains("Dropped") {
            self.dropped_weapons
                .entry(ent.index)
                .or_insert_with(|| name.to_string());
        } else if name.contains("GameRules") {
            self.rules.entity = Some(ent.clone());
        }
    }

    pub fn handle_event<E: 'static>(&mut self, event: &E) {
        let any = event as &dyn std::any::Any;
        if let Some(cv) = any.downcast_ref::<crate::events::ConVarsUpdated>() {
            for (k, v) in &cv.updated_con_vars {
                self.rules.con_vars.insert(k.clone(), v.clone());
            }
        } else if let Some(re) = any.downcast_ref::<crate::events::RoundEnd>() {
            let _ = re;
            self.total_rounds_played += 1;
        } else if let Some(ge) = any.downcast_ref::<crate::events::GamePhaseChanged>() {
            self.game_phase = ge.new_game_phase;
        } else if let Some(wu) = any.downcast_ref::<crate::events::IsWarmupPeriodChanged>() {
            self.is_warmup_period = wu.new_is_warmup_period;
        } else if let Some(ft) = any.downcast_ref::<crate::events::RoundFreezetimeChanged>() {
            self.is_freezetime = ft.new_is_freezetime;
        } else if let Some(ms) = any.downcast_ref::<crate::events::MatchStartedChanged>() {
            self.is_match_started = ms.new_is_started;
        } else if let Some(ot) = any.downcast_ref::<crate::events::OvertimeNumberChanged>() {
            self.overtime_count = ot.new_count;
        } else if let Some(ev) = any.downcast_ref::<crate::parser::EntityEvent>() {
            use crate::sendtables::EntityOp;
            if ev.op.contains(EntityOp::DELETED) {
                self.remove_entity(ev.entity.index);
                self.projectile_owners.remove(&ev.entity.index);
                self.dropped_weapons.remove(&ev.entity.index);
            } else if ev.op.contains(EntityOp::CREATED) {
                self.add_entity(ev.entity.clone());
                self.update_special_entities(&ev.entity);
            } else if ev.op.contains(EntityOp::UPDATED) {
                self.add_entity(ev.entity.clone());
                self.update_special_entities(&ev.entity);
            }
        } else if any.is::<crate::events::FrameDone>() {
            if let Some(fb) = self.flying_flashbangs.first() {
                if fb.exploded_frame > 0 && fb.exploded_frame < self.ingame_tick {
                    self.flying_flashbangs.remove(0);
                }
            }
        }
    }

    pub fn handle_net_message<M: 'static>(&mut self, msg: &M) {
        let any = msg as &dyn Any;
        if any.is::<proto_msg::CsvcMsgServerInfo>()
            || any.is::<proto_msg::CsvcMsgSendTable>()
            || any.is::<proto_msg::CsvcMsgClassInfo>()
            || any.is::<proto_msg::CsvcMsgSetPause>()
            || any.is::<proto_msg::CsvcMsgCreateStringTable>()
            || any.is::<proto_msg::CsvcMsgUpdateStringTable>()
            || any.is::<proto_msg::CsvcMsgVoiceInit>()
            || any.is::<proto_msg::CsvcMsgVoiceData>()
            || any.is::<proto_msg::CsvcMsgPrint>()
            || any.is::<proto_msg::CsvcMsgSounds>()
            || any.is::<proto_msg::CsvcMsgSetView>()
            || any.is::<proto_msg::CsvcMsgFixAngle>()
            || any.is::<proto_msg::CsvcMsgCrosshairAngle>()
            || any.is::<proto_msg::CsvcMsgBspDecal>()
            || any.is::<proto_msg::CsvcMsgSplitScreen>()
            || any.is::<proto_msg::CsvcMsgUserMessage>()
            || any.is::<proto_msg::CsvcMsgEntityMsg>()
            || any.is::<proto_msg::CsvcMsgGameEvent>()
            || any.is::<proto_msg::CsvcMsgPacketEntities>()
            || any.is::<proto_msg::CsvcMsgTempEntities>()
            || any.is::<proto_msg::CsvcMsgPrefetch>()
            || any.is::<proto_msg::CsvcMsgMenu>()
            || any.is::<proto_msg::CsvcMsgGameEventList>()
            || any.is::<proto_msg::CsvcMsgGetCvarValue>()
            || any.is::<proto_msg::CsvcMsgPaintmapData>()
            || any.is::<proto_msg::CsvcMsgCmdKeyValues>()
            || any.is::<proto_msg::CsvcMsgEncryptedData>()
            || any.is::<proto_msg::CsvcMsgHltvReplay>()
            || any.is::<proto_msg::CsvcMsgBroadcastCommand>()
        {
            // currently no game state updates implemented
        }
    }
}
