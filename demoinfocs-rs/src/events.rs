use crate::proto::msg::{self, csvc_msg_game_event};
use crate::sendtables::entity::Vector;

use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct Player;

#[derive(Clone, Debug, Default)]
pub struct Equipment;

#[derive(Clone, Debug, Default)]
pub struct TeamState;

#[derive(Clone, Debug, Default)]
pub struct GrenadeProjectile;

#[derive(Clone, Debug, Default)]
pub struct Hostage;

#[derive(Clone, Debug, Default)]
pub struct Inferno;

#[derive(Clone, Debug, Default)]
pub struct PlayerInfoData;

pub type EquipmentType = u32;
pub type Team = u8;
pub type HostageState = u32;
pub type GamePhase = u8;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum RoundEndReason {
    StillInProgress = 0,
    TargetBombed = 1,
    VIPEscaped = 2,
    VIPKilled = 3,
    TerroristsEscaped = 4,
    CTStoppedEscape = 5,
    TerroristsStopped = 6,
    BombDefused = 7,
    CTWin = 8,
    TerroristsWin = 9,
    Draw = 10,
    HostagesRescued = 11,
    TargetSaved = 12,
    HostagesNotRescued = 13,
    TerroristsNotEscaped = 14,
    VIPNotEscaped = 15,
    GameStart = 16,
    TerroristsSurrender = 17,
    CTSurrender = 18,
    TerroristsPlanted = 19,
    CTsReachedHostage = 20,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum RoundMVPReason {
    MostEliminations = 1,
    BombDefused = 2,
    BombPlanted = 3,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum HitGroup {
    Generic = 0,
    Head = 1,
    Chest = 2,
    Stomach = 3,
    LeftArm = 4,
    RightArm = 5,
    LeftLeg = 6,
    RightLeg = 7,
    Neck = 8,
    Gear = 10,
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum WarnType {
    Undefined = 0,
    BombsiteUnknown,
    TeamSwapPlayerNil,
    GameEventBeforeDescriptors,
    UnknownDemoCommandMessageType,
    MissingNetMessageDecryptionKey,
    CantReadEncryptedNetMessage,
    UnknownEquipmentIndex,
    MissingItemDefinitionIndex,
    StringTableParsingFailure,
    PacketEntitiesPanic,
}

#[derive(Clone, Debug)]
pub struct FrameDone;

#[derive(Clone, Debug)]
pub struct POVRecordingPlayerDetected {
    pub player_slot: i32,
    pub player_info: PlayerInfoData,
}

#[derive(Clone, Debug)]
pub struct MatchStart;

#[derive(Clone, Debug)]
pub struct RoundStart {
    pub time_limit: i32,
    pub frag_limit: i32,
    pub objective: String,
}

impl Default for RoundStart {
    fn default() -> Self {
        Self {
            time_limit: 0,
            frag_limit: 0,
            objective: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoundFreezetimeEnd;

#[derive(Clone, Debug)]
pub struct RoundFreezetimeChanged {
    pub old_is_freezetime: bool,
    pub new_is_freezetime: bool,
}

#[derive(Clone, Debug)]
pub struct RoundEnd {
    pub message: String,
    pub reason: RoundEndReason,
    pub winner: Team,
    pub winner_state: Option<TeamState>,
    pub loser_state: Option<TeamState>,
}

#[derive(Clone, Debug)]
pub struct RoundEndOfficial;

#[derive(Clone, Debug)]
pub struct RoundMVPAnnouncement {
    pub player: Option<Player>,
    pub reason: RoundMVPReason,
}

#[derive(Clone, Debug)]
pub struct AnnouncementMatchStarted;

#[derive(Clone, Debug)]
pub struct AnnouncementLastRoundHalf;

#[derive(Clone, Debug)]
pub struct AnnouncementFinalRound;

#[derive(Clone, Debug)]
pub struct AnnouncementWinPanelMatch;

#[derive(Clone, Debug)]
pub struct RoundAnnounceFinal;

#[derive(Clone, Debug)]
pub struct RoundAnnounceLastRoundHalf;

#[derive(Clone, Debug)]
pub struct RoundAnnounceMatchPoint;

#[derive(Clone, Debug)]
pub struct RoundAnnounceMatchStart;

#[derive(Clone, Debug)]
pub struct RoundAnnounceWarmup;

#[derive(Clone, Debug)]
pub struct RoundEndUploadStats;

#[derive(Clone, Debug)]
pub struct Footstep {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerTeamChange {
    pub player: Option<Player>,
    pub new_team_state: Option<TeamState>,
    pub old_team_state: Option<TeamState>,
    pub new_team: Team,
    pub old_team: Team,
    pub silent: bool,
    pub is_bot: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerJump {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerSound {
    pub player: Option<Player>,
    pub radius: i32,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct Kill {
    pub weapon: Option<Equipment>,
    pub victim: Option<Player>,
    pub killer: Option<Player>,
    pub assister: Option<Player>,
    pub penetrated_objects: i32,
    pub is_headshot: bool,
    pub assisted_flash: bool,
    pub attacker_blind: bool,
    pub no_scope: bool,
    pub through_smoke: bool,
    pub distance: f32,
}

#[derive(Clone, Debug)]
pub struct BotTakenOver {
    pub taker: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct WeaponFire {
    pub shooter: Option<Player>,
    pub weapon: Option<Equipment>,
}

#[derive(Clone, Debug)]
pub struct WeaponReload {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct GrenadeEvent {
    pub grenade_type: EquipmentType,
    pub grenade: Option<Equipment>,
    pub position: Vector,
    pub thrower: Option<Player>,
    pub grenade_entity_id: i32,
}

impl Default for GrenadeEvent {
    fn default() -> Self {
        Self {
            grenade_type: 0,
            grenade: None,
            position: Vector::default(),
            thrower: None,
            grenade_entity_id: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeExplode {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct FlashExplode {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct DecoyStart {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct DecoyExpired {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct SmokeStart {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct SmokeExpired {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct FireGrenadeStart {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct FireGrenadeExpired {
    pub inner: GrenadeEvent,
}

#[derive(Clone, Debug)]
pub struct GrenadeProjectileBounce {
    pub projectile: Option<GrenadeProjectile>,
    pub bounce_nr: i32,
}

#[derive(Clone, Debug)]
pub struct GrenadeProjectileThrow {
    pub projectile: Option<GrenadeProjectile>,
}

#[derive(Clone, Debug)]
pub struct GrenadeProjectileDestroy {
    pub projectile: Option<GrenadeProjectile>,
}

#[derive(Clone, Debug)]
pub struct PlayerFlashed {
    pub player: Option<Player>,
    pub attacker: Option<Player>,
    pub projectile: Option<GrenadeProjectile>,
}

#[derive(Clone, Debug)]
pub enum Bombsite {
    Unknown,
    A,
    B,
}

#[derive(Clone, Debug)]
pub struct BombEvent {
    pub player: Option<Player>,
    pub site: Bombsite,
}

#[derive(Clone, Debug)]
pub struct BombPlantBegin {
    pub inner: BombEvent,
}

#[derive(Clone, Debug)]
pub struct BombPlantAborted {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct BombPlanted {
    pub inner: BombEvent,
}

#[derive(Clone, Debug)]
pub struct BombDefused {
    pub inner: BombEvent,
}

#[derive(Clone, Debug)]
pub struct BombExplode {
    pub inner: BombEvent,
}

#[derive(Clone, Debug)]
pub struct BombDefuseStart {
    pub player: Option<Player>,
    pub has_kit: bool,
}

#[derive(Clone, Debug)]
pub struct BombDefuseAborted {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct BombDropped {
    pub player: Option<Player>,
    pub entity_id: i32,
}

#[derive(Clone, Debug)]
pub struct BombPickup {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct BombBeep {
    pub inner: BombEvent,
}

#[derive(Clone, Debug)]
pub struct HostageRescued {
    pub player: Option<Player>,
    pub hostage: Option<Hostage>,
}

#[derive(Clone, Debug)]
pub struct HostageRescuedAll;

#[derive(Clone, Debug)]
pub struct HostageHurt {
    pub player: Option<Player>,
    pub hostage: Option<Hostage>,
}

#[derive(Clone, Debug)]
pub struct HostageKilled {
    pub killer: Option<Player>,
    pub hostage: Option<Hostage>,
}

#[derive(Clone, Debug)]
pub struct HostageStateChanged {
    pub old_state: HostageState,
    pub new_state: HostageState,
    pub hostage: Option<Hostage>,
}

#[derive(Clone, Debug)]
pub struct BulletDamage {
    pub attacker: Option<Player>,
    pub victim: Option<Player>,
    pub distance: f32,
    pub damage_dir_x: f32,
    pub damage_dir_y: f32,
    pub damage_dir_z: f32,
    pub num_penetrations: i32,
    pub is_no_scope: bool,
    pub is_attacker_in_air: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerHurt {
    pub player: Option<Player>,
    pub attacker: Option<Player>,
    pub health: i32,
    pub armor: i32,
    pub weapon: Option<Equipment>,
    pub weapon_string: String,
    pub health_damage: i32,
    pub armor_damage: i32,
    pub health_damage_taken: i32,
    pub armor_damage_taken: i32,
    pub hit_group: HitGroup,
}

#[derive(Clone, Debug)]
pub struct PlayerConnect {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct BotConnect {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerDisconnected {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerNameChange {
    pub player: Option<Player>,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Clone, Debug)]
pub struct PlayerSpawn {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerSpawned {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerTeam {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerPing {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerPingStop {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerGivenC4 {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct PlayerFallDamage {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct StringTablePlayerUpdateApplied {
    pub player: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct SayText {
    pub ent_idx: i32,
    pub text: String,
    pub is_chat: bool,
    pub is_chat_all: bool,
}

#[derive(Clone, Debug)]
pub struct SayText2 {
    pub ent_idx: i32,
    pub msg_name: String,
    pub params: Vec<String>,
    pub is_chat: bool,
    pub is_chat_all: bool,
}

#[derive(Clone, Debug)]
pub struct TickRateInfoAvailable {
    pub tick_rate: f64,
    pub tick_time: Duration,
}

#[derive(Clone, Debug)]
pub struct ParserWarn {
    pub message: String,
    pub r#type: WarnType,
}

#[derive(Clone, Debug)]
pub struct GenericGameEvent {
    pub name: String,
    pub data: HashMap<String, csvc_msg_game_event::KeyT>,
}

#[derive(Clone, Debug)]
pub struct InfernoStart {
    pub inferno: Option<Inferno>,
}

#[derive(Clone, Debug)]
pub struct InfernoExpired {
    pub inferno: Option<Inferno>,
}

#[derive(Clone, Debug)]
pub struct ScoreUpdated {
    pub old_score: i32,
    pub new_score: i32,
    pub team_state: Option<TeamState>,
}

#[derive(Clone, Debug)]
pub struct GamePhaseChanged {
    pub old_game_phase: GamePhase,
    pub new_game_phase: GamePhase,
}

#[derive(Clone, Debug)]
pub struct TeamSideSwitch;

#[derive(Clone, Debug)]
pub struct GameHalfEnded;

#[derive(Clone, Debug)]
pub struct MatchStartedChanged {
    pub old_is_started: bool,
    pub new_is_started: bool,
}

#[derive(Clone, Debug)]
pub struct IsWarmupPeriodChanged {
    pub old_is_warmup_period: bool,
    pub new_is_warmup_period: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerSpottersChanged {
    pub spotted: Option<Player>,
}

#[derive(Clone, Debug)]
pub struct ConVarsUpdated {
    pub updated_con_vars: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct RoundImpactScoreData {
    pub raw_message: Option<msg::CcsUsrMsgRoundImpactScoreData>,
}

#[derive(Clone, Debug)]
pub struct PlayerInfo {
    pub index: i32,
    pub info: PlayerInfoData,
}

#[derive(Clone, Debug)]
pub struct OvertimeNumberChanged {
    pub old_count: i32,
    pub new_count: i32,
}

#[derive(Clone, Debug)]
pub struct ItemRefund {
    pub player: Option<Player>,
    pub weapon: Option<Equipment>,
}

#[derive(Clone, Debug)]
pub struct DataTablesParsed;

pub struct TeamClanNameUpdated {
    pub old_name: String,
    pub new_name: String,
    pub team_state: Option<TeamState>,
}

pub struct AnnouncePhaseEnd;

#[derive(Clone, Debug)]
pub struct BuytimeEnded;

#[derive(Clone, Debug)]
pub struct ChoppersIncomingWarning;

#[derive(Clone, Debug)]
pub struct CsIntermission;

#[derive(Clone, Debug)]
pub struct CsMatchEndRestart;

#[derive(Clone, Debug)]
pub struct CsPreRestart;

#[derive(Clone, Debug)]
pub struct CsRoundFinalBeep;

#[derive(Clone, Debug)]
pub struct CsRoundStartBeep;

#[derive(Clone, Debug)]
pub struct CsWinPanelMatch;

#[derive(Clone, Debug)]
pub struct CsWinPanelRound;

#[derive(Clone, Debug)]
pub struct EnterBombzone;

#[derive(Clone, Debug)]
pub struct ExitBombzone;

#[derive(Clone, Debug)]
pub struct EnterBuyzone;

#[derive(Clone, Debug)]
pub struct ExitBuyzone;

#[derive(Clone, Debug)]
pub struct EntityVisible;

#[derive(Clone, Debug)]
pub struct FirstBombsIncomingWarning;

#[derive(Clone, Debug)]
pub struct HltvChase;

#[derive(Clone, Debug)]
pub struct HltvFixed;

#[derive(Clone, Debug)]
pub struct HltvMessage;

#[derive(Clone, Debug)]
pub struct HltvStatus;

#[derive(Clone, Debug)]
pub struct HostageFollows;

#[derive(Clone, Debug)]
pub struct HostnameChanged;

#[derive(Clone, Debug)]
pub struct JoinTeamFailed;

#[derive(Clone, Debug)]
pub struct OtherDeath;

#[derive(Clone, Debug)]
pub struct PlayerBlind;

#[derive(Clone, Debug)]
pub struct ShowSurvivalRespawnStatus;

#[derive(Clone, Debug)]
pub struct SurvivalParadropSpawn;

#[derive(Clone, Debug)]
pub struct SwitchTeam;

#[derive(Clone, Debug)]
pub struct WeaponFireOnEmpty;

#[derive(Clone, Debug)]
pub struct WeaponZoom;

#[derive(Clone, Debug)]
pub struct WeaponZoomRifle;
=======
pub struct AmmoPickup;

#[derive(Clone, Debug)]
pub struct ItemEquip;

#[derive(Clone, Debug)]
pub struct ItemPickup;

#[derive(Clone, Debug)]
pub struct ItemPickupSlerp;

#[derive(Clone, Debug)]
pub struct ItemRemove;

#[derive(Clone, Debug)]
pub struct InspectWeapon;

#[derive(Clone, Debug)]
pub struct ServerCvar;

#[derive(Clone, Debug)]
pub struct VoteCast;

#[derive(Clone, Debug)]
pub struct TournamentReward;
