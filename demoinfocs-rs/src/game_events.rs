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
            | _ => {},
        }
    }
}
