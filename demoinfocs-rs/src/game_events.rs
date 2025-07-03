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

    pub fn handle_game_event<R: Read>(
        &mut self,
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
            | _ => {},
        }
    }
}
