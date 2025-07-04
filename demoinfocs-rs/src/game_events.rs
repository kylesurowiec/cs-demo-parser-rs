use std::collections::HashMap;
use std::io::Read;

use crate::events;
use crate::parser::Parser;
use crate::proto::msg::all as msg;

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
            | "hostage_follows" => parser.dispatch_event(events::HostageFollows),
            | "hostname_changed" => parser.dispatch_event(events::HostnameChanged),
            | "jointeam_failed" => parser.dispatch_event(events::JoinTeamFailed),
            | "other_death" => parser.dispatch_event(events::OtherDeath),
            | "player_blind" => parser.dispatch_event(events::PlayerBlind),
            | "show_survival_respawn_status" => {
                parser.dispatch_event(events::ShowSurvivalRespawnStatus)
            },
            | "survival_paradrop_spawn" => parser.dispatch_event(events::SurvivalParadropSpawn),
            | "switch_team" => parser.dispatch_event(events::SwitchTeam),
            | "weapon_fire_on_empty" => parser.dispatch_event(events::WeaponFireOnEmpty),
            | "weapon_zoom" => parser.dispatch_event(events::WeaponZoom),
            | "weapon_zoom_rifle" => parser.dispatch_event(events::WeaponZoomRifle),
            | _ => {},
        }
    }
}
