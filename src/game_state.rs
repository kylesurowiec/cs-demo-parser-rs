use std::any::Any;
use std::collections::HashMap;

use crate::common::{Bomb, Equipment, GrenadeProjectile, Hostage, Inferno, Player, Team};
use crate::game_rules::GameRules;
use crate::match_info::MatchInfo;
use crate::proto::msg::cs_demo_parser_rs as proto_msg;
use crate::proto::msgs2::CMsgPlayerInfo;
use crate::sendtables2::Entity;
use prost::Message;

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
    pub match_info: MatchInfo,
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

    /// Returns all currently active grenade projectiles.
    pub fn active_grenades(&self) -> Vec<&GrenadeProjectile> {
        self.grenade_projectiles.values().collect()
    }

    /// Records the current position of a grenade projectile.
    pub fn track_grenade_position(
        &mut self,
        entity_id: i32,
        position: crate::sendtables::entity::Vector,
        frame: i32,
        time: std::time::Duration,
    ) {
        if let Some(g) = self.grenade_projectiles.get_mut(&entity_id) {
            g.track_position(position, frame, time);
        }
    }

    pub fn infernos(&self) -> &HashMap<i32, Inferno> {
        &self.infernos
    }

    /// Returns all currently active infernos.
    pub fn active_infernos(&self) -> Vec<&Inferno> {
        self.infernos.values().collect()
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

    pub fn match_info(&self) -> &MatchInfo {
        &self.match_info
    }

    pub fn map_name(&self) -> Option<&str> {
        self.match_info.map.as_deref()
    }

    pub fn match_id(&self) -> Option<u64> {
        self.match_info.match_id
    }

    pub fn server_version(&self) -> Option<u32> {
        self.match_info.server_version
    }

    pub fn match_settings(&self) -> &std::collections::HashMap<String, String> {
        &self.rules.con_vars
    }

    pub fn round_time(&self) -> Option<std::time::Duration> {
        self.rules.round_time()
    }

    pub fn freeze_time(&self) -> Option<std::time::Duration> {
        self.rules.freeze_time()
    }

    pub fn bomb_time(&self) -> Option<std::time::Duration> {
        self.rules.bomb_time()
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

    pub fn apply_userinfo_table(&mut self, table: &crate::stringtables::StringTable) {
        if table.name.eq_ignore_ascii_case("userinfo") {
            for (idx, entry) in &table.entries {
                if entry.user_data.is_empty() {
                    continue;
                }
                if let Ok(info) = CMsgPlayerInfo::decode(&entry.user_data[..]) {
                    let user_id = info.userid.unwrap_or(*idx);
                    let steam_id = info.steamid.or(info.xuid).unwrap_or(0);
                    let name = info.name.unwrap_or_default();
                    let is_bot = info.fakeplayer.unwrap_or(false);
                    let p = self
                        .players_by_user_id
                        .entry(user_id)
                        .or_insert_with(crate::common::Player::default);
                    p.user_id = user_id;
                    p.name = name;
                    p.steam_id64 = steam_id;
                    p.is_bot = is_bot;
                    p.is_connected = true;
                    if p.entity_id != 0 {
                        self.players_by_entity_id.insert(p.entity_id, p.clone());
                    }
                }
            }
        }
    }

    fn update_player_from_entity(&mut self, ent: &Entity) {
        if !ent.class.name.to_lowercase().contains("player") {
            return;
        }
        let p = self
            .players_by_entity_id
            .entry(ent.index)
            .or_insert_with(crate::common::Player::default);
        p.entity_id = ent.index;
        if p.user_id == 0 {
            p.user_id = ent.index;
        }
        // team information might be available via entity properties in a
        // complete implementation. This simplified version does not decode
        // properties and leaves the team unchanged.
        self.players_by_user_id.insert(p.user_id, p.clone());
    }

    fn update_special_entities(&mut self, ent: &Entity) {
        let name = ent.class.name.as_str();
        if name.contains("Projectile") {
            self.projectile_owners.entry(ent.index).or_insert(0);
            self.grenade_projectiles
                .entry(ent.index)
                .or_insert_with(|| {
                    let mut g = crate::common::new_grenade_projectile();
                    g.entity = Some(ent.clone());
                    g
                });
        } else if name.contains("Inferno") {
            self.infernos
                .entry(ent.index)
                .or_insert_with(|| crate::common::Inferno {
                    entity: Some(ent.clone()),
                    ..Default::default()
                });
        } else if name.contains("DroppedWeapon") || name.contains("Dropped") {
            self.dropped_weapons
                .entry(ent.index)
                .or_insert_with(|| name.to_string());
        } else if let Some(eq) = self.equipment_mapping.get(name) {
            self.weapons.entry(ent.index).or_insert_with(|| Equipment {
                equipment_type: *eq,
                entity: None,
                original_string: name.to_string(),
                unique_id: ent.index as i64,
                position: Default::default(),
            });
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
                self.weapons.remove(&ev.entity.index);
                self.projectile_owners.remove(&ev.entity.index);
                self.dropped_weapons.remove(&ev.entity.index);
                self.grenade_projectiles.remove(&ev.entity.index);
                self.infernos.remove(&ev.entity.index);
            } else if ev.op.contains(EntityOp::CREATED) || ev.op.contains(EntityOp::UPDATED) {
                self.add_entity(ev.entity.clone());
                self.update_special_entities(&ev.entity);
                self.update_player_from_entity(&ev.entity);
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
        if let Some(info) = any.downcast_ref::<proto_msg::CsvcMsgServerInfo>() {
            self.match_info.map = info.map_name.clone();
        } else if any.is::<proto_msg::CsvcMsgSendTable>()
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
            || any.is::<proto_msg::CnetMsgNop>()
            || any.is::<proto_msg::CnetMsgDisconnect>()
            || any.is::<proto_msg::CnetMsgFile>()
            || any.is::<proto_msg::CnetMsgSplitScreenUser>()
            || any.is::<proto_msg::CnetMsgTick>()
            || any.is::<proto_msg::CnetMsgStringCmd>()
            || any.is::<proto_msg::CnetMsgSetConVar>()
            || any.is::<proto_msg::CnetMsgSignonState>()
            || any.is::<proto_msg::CnetMsgPlayerAvatarData>()
        {
            // currently no game state updates implemented
        }
    }
}
