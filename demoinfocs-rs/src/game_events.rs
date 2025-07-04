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
            // Grenade events
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
            // Bomb events
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
            | _ => {},
        }
    }
}