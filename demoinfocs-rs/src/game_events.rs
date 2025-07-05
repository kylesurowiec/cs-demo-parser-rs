use crate::proto::msg;
use std::collections::HashMap;
use std::io::Read;

use crate::events;
use crate::parser::Parser;
// Update this import to match the actual structure of your proto::msg module.
// For example, if you want to import everything from msg, use:
// Use the correct import for the message types you need:

struct Descriptor {
    name: String,
}

#[derive(Default)]
pub struct GameEventHandler {
    descriptors: HashMap<i32, Descriptor>,
}

impl GameEventHandler {
    pub fn new() -> Self {
        Self {
            descriptors: HashMap::new(),
        }
    }

    pub fn handle_game_event_list(&mut self, list: &msg::CsvcMsgGameEventList) {
        self.descriptors.clear();
        for desc in &list.descriptors {
            if let (Some(id), Some(name)) = (desc.eventid, desc.name.as_ref()) {
                self.descriptors
                    .insert(id, Descriptor { name: name.clone() });
            }
        }
    }

    pub fn descriptor_name(&self, id: i32) -> Option<&str> {
        self.descriptors.get(&id).map(|d| d.name.as_str())
    }

    pub fn handle_game_event<R: Read>(
        &self,
        parser: &mut Parser<R>,
        event: &msg::CsvcMsgGameEvent,
    ) {
        let id = match event.eventid {
            | Some(v) => v,
            | None => return,
        };
        let desc = match self.descriptors.get(&id) {
            | Some(d) => d,
            | None => return,
        };

        match desc.name.as_str() {
            | "begin_new_match" => parser.dispatch_event(events::MatchStart),
            | "round_start" => parser.dispatch_event(events::RoundStart::default()),
            | "round_end" => parser.dispatch_event(events::RoundEnd {
                message: String::new(),
                reason: events::RoundEndReason::StillInProgress,
                winner: 0,
                winner_state: None,
                loser_state: None,
            }),
            | "round_announce_final" => parser.dispatch_event(events::RoundAnnounceFinal),
            | "round_announce_last_round_half" => {
                parser.dispatch_event(events::RoundAnnounceLastRoundHalf)
            },
            | "round_announce_match_point" => {
                parser.dispatch_event(events::RoundAnnounceMatchPoint)
            },
            | "round_announce_match_start" => {
                parser.dispatch_event(events::RoundAnnounceMatchStart)
            },
            | "round_announce_warmup" => parser.dispatch_event(events::RoundAnnounceWarmup),
            | "round_end_upload_stats" => parser.dispatch_event(events::RoundEndUploadStats),
            | "round_mvp" => parser.dispatch_event(events::RoundMVPAnnouncement {
                player: None,
                reason: events::RoundMVPReason::MostEliminations,
            }),
            | "round_freeze_end" => parser.dispatch_event(events::RoundFreezetimeEnd),
            | "round_officially_ended" => parser.dispatch_event(events::RoundEndOfficial),
            | "player_connect" | "player_connect_full" => {
                parser.dispatch_event(events::PlayerConnect { player: None })
            },
            | "player_disconnect" => {
                parser.dispatch_event(events::PlayerDisconnected { player: None })
            },
            | "player_changename" => parser.dispatch_event(events::PlayerNameChange {
                player: None,
                old_name: String::new(),
                new_name: String::new(),
            }),
            | "player_spawn" => parser.dispatch_event(events::PlayerSpawn { player: None }),
            | "player_spawned" => parser.dispatch_event(events::PlayerSpawned { player: None }),
            | "player_team" => parser.dispatch_event(events::PlayerTeam { player: None }),
            | "player_ping" => parser.dispatch_event(events::PlayerPing { player: None }),
            | "player_ping_stop" => parser.dispatch_event(events::PlayerPingStop { player: None }),
            | "player_falldamage" => {
                parser.dispatch_event(events::PlayerFallDamage { player: None })
            },
            | "player_given_c4" => parser.dispatch_event(events::PlayerGivenC4 { player: None }),
            | "player_jump" => parser.dispatch_event(events::PlayerJump { player: None }),
            | "player_footstep" => parser.dispatch_event(events::Footstep { player: None }),
            | "flashbang_detonate" => parser.dispatch_event(events::FlashExplode {
                inner: events::GrenadeEvent::default(),
            }),
            | "hegrenade_detonate" => parser.dispatch_event(events::HeExplode {
                inner: events::GrenadeEvent::default(),
            }),
            | "decoy_started" => parser.dispatch_event(events::DecoyStart {
                inner: events::GrenadeEvent::default(),
            }),
            | "decoy_detonate" => parser.dispatch_event(events::DecoyExpired {
                inner: events::GrenadeEvent::default(),
            }),
            | "smokegrenade_detonate" => parser.dispatch_event(events::SmokeStart {
                inner: events::GrenadeEvent::default(),
            }),
            | "smokegrenade_expired" => parser.dispatch_event(events::SmokeExpired {
                inner: events::GrenadeEvent::default(),
            }),
            | "inferno_startburn" => parser.dispatch_event(events::FireGrenadeStart {
                inner: events::GrenadeEvent::default(),
            }),
            | "inferno_expire" => parser.dispatch_event(events::FireGrenadeExpired {
                inner: events::GrenadeEvent::default(),
            }),
            | "bomb_beginplant" => parser.dispatch_event(events::BombPlantBegin {
                inner: events::BombEvent {
                    player: None,
                    site: events::Bombsite::Unknown,
                },
            }),
            | "bomb_begindefuse" => parser.dispatch_event(events::BombDefuseStart {
                player: None,
                has_kit: false,
            }),
            | "bomb_defused" => parser.dispatch_event(events::BombDefused {
                inner: events::BombEvent {
                    player: None,
                    site: events::Bombsite::Unknown,
                },
            }),
            | "bomb_exploded" => parser.dispatch_event(events::BombExplode {
                inner: events::BombEvent {
                    player: None,
                    site: events::Bombsite::Unknown,
                },
            }),
            | "bomb_dropped" => parser.dispatch_event(events::BombDropped {
                player: None,
                entity_id: 0,
            }),
            | "bomb_pickup" => parser.dispatch_event(events::BombPickup { player: None }),
            | "bomb_planted" => parser.dispatch_event(events::BombPlanted {
                inner: events::BombEvent {
                    player: None,
                    site: events::Bombsite::Unknown,
                },
            }),
            | "bomb_beep" => parser.dispatch_event(events::BombBeep {
                inner: events::BombEvent {
                    player: None,
                    site: events::Bombsite::Unknown,
                },
            }),
            | "announce_phase_end" => parser.dispatch_event(events::AnnouncePhaseEnd),
            | "buytime_ended" => parser.dispatch_event(events::BuytimeEnded),
            | "choppers_incoming_warning" => parser.dispatch_event(events::ChoppersIncomingWarning),
            | "cs_intermission" => parser.dispatch_event(events::CsIntermission),
            | "cs_match_end_restart" => parser.dispatch_event(events::CsMatchEndRestart),
            | "cs_pre_restart" => parser.dispatch_event(events::CsPreRestart),
            | "cs_round_final_beep" => parser.dispatch_event(events::CsRoundFinalBeep),
            | "cs_round_start_beep" => parser.dispatch_event(events::CsRoundStartBeep),
            | "cs_win_panel_match" => parser.dispatch_event(events::CsWinPanelMatch),
            | "cs_win_panel_round" => parser.dispatch_event(events::CsWinPanelRound),
            | "enter_bombzone" => parser.dispatch_event(events::EnterBombzone),
            | "exit_bombzone" => parser.dispatch_event(events::ExitBombzone),
            | "enter_buyzone" => parser.dispatch_event(events::EnterBuyzone),
            | "exit_buyzone" => parser.dispatch_event(events::ExitBuyzone),
            | "entity_visible" => parser.dispatch_event(events::EntityVisible),
            | "firstbombs_incoming_warning" => {
                parser.dispatch_event(events::FirstBombsIncomingWarning)
            },
            | "hltv_chase" => parser.dispatch_event(events::HltvChase),
            | "hltv_fixed" => parser.dispatch_event(events::HltvFixed),
            | "hltv_message" => parser.dispatch_event(events::HltvMessage),
            | "hltv_status" => parser.dispatch_event(events::HltvStatus),
            | "hltv_title" => parser.dispatch_event(events::HltvTitle),
            | "hltv_versioninfo" => parser.dispatch_event(events::HltvVersionInfo),
            | "hostage_follows" => parser.dispatch_event(events::HostageFollows),
            | "hostname_changed" => parser.dispatch_event(events::HostnameChanged),
            | "jointeam_failed" => parser.dispatch_event(events::JoinTeamFailed),
            | "other_death" => parser.dispatch_event(events::OtherDeath),
            | "player_blind" => parser.dispatch_event(events::PlayerBlind),
            | "bot_takeover" => parser.dispatch_event(events::BotTakenOver { taker: None }),
            | "bullet_damage" => parser.dispatch_event(events::BulletDamage {
                attacker: None,
                victim: None,
                distance: 0.0,
                damage_dir_x: 0.0,
                damage_dir_y: 0.0,
                damage_dir_z: 0.0,
                num_penetrations: 0,
                is_no_scope: false,
                is_attacker_in_air: false,
            }),
            | "endmatch_cmm_start_reveal_items" => {
                parser.dispatch_event(events::EndmatchCmmStartRevealItems)
            },
            | "entity_killed" => parser.dispatch_event(events::EntityKilled),
            | "grenade_thrown" => parser.dispatch_event(events::GrenadeThrown),
            | "show_survival_respawn_status" => {
                parser.dispatch_event(events::ShowSurvivalRespawnStatus)
            },
            | "survival_paradrop_spawn" => parser.dispatch_event(events::SurvivalParadropSpawn),
            | "switch_team" => parser.dispatch_event(events::SwitchTeam),
            | "weapon_fire_on_empty" => parser.dispatch_event(events::WeaponFireOnEmpty),
            | "weapon_fire" => parser.dispatch_event(events::WeaponFire {
                shooter: None,
                weapon: None,
            }),
            | "weapon_reload" => parser.dispatch_event(events::WeaponReload { player: None }),
            | "weapon_zoom" => parser.dispatch_event(events::WeaponZoom),
            | "weapon_zoom_rifle" => parser.dispatch_event(events::WeaponZoomRifle),
            | "ammo_pickup" => parser.dispatch_event(events::AmmoPickup),
            | "item_equip" => parser.dispatch_event(events::ItemEquip),
            | "item_pickup" => parser.dispatch_event(events::ItemPickup),
            | "item_pickup_slerp" => parser.dispatch_event(events::ItemPickupSlerp),
            | "item_remove" => parser.dispatch_event(events::ItemRemove),
            | "inspect_weapon" => parser.dispatch_event(events::InspectWeapon),
            | "server_cvar" => parser.dispatch_event(events::ServerCvar),
            | "vote_cast" => parser.dispatch_event(events::VoteCast),
            | "tournament_reward" => parser.dispatch_event(events::TournamentReward),
            | "hostage_hurt" => parser.dispatch_event(events::HostageHurt {
                player: None,
                hostage: None,
            }),
            | "hostage_killed" => parser.dispatch_event(events::HostageKilled {
                killer: None,
                hostage: None,
            }),
            | "hostage_rescued" => parser.dispatch_event(events::HostageRescued {
                player: None,
                hostage: None,
            }),
            | "hostage_rescued_all" => parser.dispatch_event(events::HostageRescuedAll),
            | "player_activate" => parser.dispatch_event(events::PlayerActivate),
            | "player_death" => parser.dispatch_event(events::Kill {
                weapon: None,
                victim: None,
                killer: None,
                assister: None,
                penetrated_objects: 0,
                is_headshot: false,
                assisted_flash: false,
                attacker_blind: false,
                no_scope: false,
                through_smoke: false,
                distance: 0.0,
            }),
            | "player_hurt" => parser.dispatch_event(events::PlayerHurt {
                player: None,
                attacker: None,
                health: 0,
                armor: 0,
                weapon: None,
                weapon_string: String::new(),
                health_damage: 0,
                armor_damage: 0,
                health_damage_taken: 0,
                armor_damage_taken: 0,
                hit_group: events::HitGroup::Generic,
            }),
            | "player_sound" => parser.dispatch_event(events::PlayerSound {
                player: None,
                radius: 0,
                duration: std::time::Duration::from_secs(0),
            }),
            | "round_poststart" => parser.dispatch_event(events::RoundPoststart),
            | "round_prestart" => parser.dispatch_event(events::RoundPrestart),
            | "round_time_warning" => parser.dispatch_event(events::RoundTimeWarning),
            | _ => {},
        }
    }
}
