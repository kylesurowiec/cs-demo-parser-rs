use crate::bitreader::BitReader;
use crate::dispatcher::{Dispatcher, EventDispatcher, HandlerIdentifier};
use crate::game_state::GameState;
use crate::sendtables::EntityOp;
use crate::sendtables::TablesParser;
use crate::sendtables2;

pub mod datatable;

use prost::Message;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

/// Error type returned by [`Parser`] operations.
#[derive(Debug)]
pub enum ParserError {
    /// The demo stream ended prematurely.
    UnexpectedEndOfDemo,
    /// The input does not look like a valid demo file.
    InvalidFileType,
}

/// Information parsed from a demo's header.
#[derive(Default, Debug, Clone)]
pub struct DemoHeader {
    pub filestamp: String,
    pub protocol: i32,
    pub network_protocol: i32,
    pub server_name: String,
    pub client_name: String,
    pub map_name: String,
    pub game_directory: String,
    pub playback_time: f32,
    pub playback_ticks: i32,
    pub playback_frames: i32,
    pub signon_length: i32,
}

#[derive(Clone, Debug)]
pub struct EntityEvent {
    pub entity: crate::sendtables2::Entity,
    pub op: crate::sendtables::EntityOp,
}

#[derive(Clone, Debug)]
pub struct EntityCreated {
    pub entity: crate::sendtables2::Entity,
}

/// Parser for CS:GO / CS2 demo files.
pub struct Parser<R: Read> {
    bit_reader: BitReader<R>,
    event_dispatcher: Arc<EventDispatcher>,
    msg_dispatcher: Arc<EventDispatcher>,
    user_msg_dispatcher: Arc<EventDispatcher>,
    s2_tables: sendtables2::Parser,
    s1_tables: TablesParser,
    server_classes: Vec<crate::sendtables::ServerClass>,
    equipment_mapping: HashMap<String, crate::common::EquipmentType>,
    game_state: GameState,
    current_frame: i32,
    cancelled: bool,
    game_events: crate::game_events::GameEventHandler,
    header: Option<DemoHeader>,
}

impl<R: Read> Parser<R> {
    /// Creates a new [`Parser`] from the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            bit_reader: BitReader::new_large(reader),
            event_dispatcher: EventDispatcher::new(),
            msg_dispatcher: EventDispatcher::new(),
            user_msg_dispatcher: EventDispatcher::new(),
            s2_tables: sendtables2::Parser::new(),
            s1_tables: TablesParser::new(),
            server_classes: Vec::new(),
            equipment_mapping: HashMap::new(),
            game_state: GameState::default(),
            current_frame: 0,
            cancelled: false,
            game_events: crate::game_events::GameEventHandler::new(),
            header: None,
        }
    }

    pub fn register_event_handler<E, F>(&self, handler: F) -> HandlerIdentifier
    where
        E: Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static,
    {
        self.event_dispatcher.register_handler::<E, F>(handler)
    }

    pub fn register_net_message_handler<M, F>(&self, handler: F) -> HandlerIdentifier
    where
        M: Send + Sync + 'static,
        F: Fn(&M) + Send + Sync + 'static,
    {
        self.msg_dispatcher.register_handler::<M, F>(handler)
    }

    pub fn register_user_message_handler<M, F>(&self, handler: F) -> HandlerIdentifier
    where
        M: Send + Sync + 'static,
        F: Fn(&M) + Send + Sync + 'static,
    {
        self.user_msg_dispatcher.register_handler::<M, F>(handler)
    }

    pub fn register_on_entity<F>(&self, handler: F) -> HandlerIdentifier
    where
        F: Fn(&EntityEvent) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .register_handler::<EntityEvent, F>(handler)
    }

    pub fn register_on_entity_created<F>(&self, handler: F) -> HandlerIdentifier
    where
        F: Fn(&crate::sendtables2::Entity) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .register_handler::<EntityEvent, _>(move |ev| {
                if ev.op.contains(crate::sendtables::EntityOp::CREATED) {
                    handler(&ev.entity);
                }
            })
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    fn update_equipment_mapping_from_classes(&mut self) {
        self.equipment_mapping = datatable::build_equipment_mapping(&self.server_classes);
        self.game_state.equipment_mapping = self.equipment_mapping.clone();
    }

    pub fn dispatch_event<E>(&mut self, event: E)
    where
        E: Send + Sync + 'static,
    {
        self.game_state_mut().handle_event(&event);
        self.event_dispatcher.dispatch(event);
    }

    pub fn dispatch_net_message<M>(&mut self, msg: M)
    where
        M: Send + Sync + 'static,
    {
        self.game_state_mut().handle_net_message(&msg);
        self.msg_dispatcher.dispatch(msg);
    }

    pub fn unregister_event_handler(&self, id: HandlerIdentifier) {
        self.event_dispatcher.unregister_handler(id);
    }

    pub fn unregister_net_message_handler(&self, id: HandlerIdentifier) {
        self.msg_dispatcher.unregister_handler(id);
    }

    pub fn unregister_user_message_handler(&self, id: HandlerIdentifier) {
        self.user_msg_dispatcher.unregister_handler(id);
    }

    pub fn server_classes(&self) -> &crate::sendtables::ServerClasses {
        &self.server_classes
    }

    pub fn header(&self) -> Option<DemoHeader> {
        self.header.clone()
    }

    pub fn current_frame(&self) -> i32 {
        self.current_frame
    }

    pub fn current_time(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f32(
            self.current_frame as f32 * self.tick_time().as_secs_f32(),
        )
    }

    pub fn tick_rate(&self) -> f64 {
        self.header
            .as_ref()
            .and_then(|h| {
                if h.playback_time > 0.0 {
                    Some(h.playback_ticks as f64 / h.playback_time as f64)
                } else {
                    None
                }
            })
            .unwrap_or(0.0)
    }

    pub fn tick_time(&self) -> std::time::Duration {
        if let Some(rate) = match self.tick_rate() {
            | r if r > 0.0 => Some(r),
            | _ => None,
        } {
            std::time::Duration::from_secs_f64(1.0 / rate)
        } else {
            std::time::Duration::from_secs(0)
        }
    }

    pub fn progress(&self) -> f32 {
        if let Some(h) = &self.header {
            if h.playback_frames > 0 {
                return self.current_frame as f32 / h.playback_frames as f32;
            }
        }
        0.0
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn close(&mut self) -> Result<(), ()> {
        self.cancelled = true;
        Ok(())
    }

    pub fn dispatch_user_message<M>(&self, msg: M)
    where
        M: Send + Sync + 'static,
    {
        self.user_msg_dispatcher.dispatch(msg);
    }

    /// Parses the demo header if it hasn't been read yet.
    pub fn parse_header(&mut self) -> Result<DemoHeader, ParserError> {
        if let Some(h) = &self.header {
            return Ok(h.clone());
        }

        let mut header = DemoHeader::default();
        header.filestamp = self.bit_reader.read_c_string(8);
        match header.filestamp.as_str() {
            | "HL2DEMO" | "PBDEMS2" => {},
            | _ => return Err(ParserError::InvalidFileType),
        }

        header.protocol = self.bit_reader.read_signed_int(32);
        header.network_protocol = self.bit_reader.read_signed_int(32);
        header.server_name = self.bit_reader.read_c_string(260);
        header.client_name = self.bit_reader.read_c_string(260);
        header.map_name = self.bit_reader.read_c_string(260);
        header.game_directory = self.bit_reader.read_c_string(260);
        header.playback_time = self.bit_reader.read_float();
        header.playback_ticks = self.bit_reader.read_signed_int(32);
        header.playback_frames = self.bit_reader.read_signed_int(32);
        header.signon_length = self.bit_reader.read_signed_int(32);

        self.header = Some(header.clone());
        Ok(header)
    }

    /// Parses the next frame of the demo. Returns `Ok(false)` if the demo
    /// contains no further frames.
    pub fn parse_next_frame(&mut self) -> Result<bool, ParserError> {
        if self.header.is_none() {
            self.parse_header()?;
        }

        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match self
                .header
                .as_ref()
                .map(|h| h.filestamp.as_str())
                .unwrap_or("")
            {
                | "HL2DEMO" => self.parse_frame_s1(),
                | "PBDEMS2" => self.parse_frame_s2(),
                | _ => Ok(false),
            }
        }))
        .unwrap_or(Err(ParserError::UnexpectedEndOfDemo))
    }

    /// Parses the demo until the end.
    pub fn parse_to_end(&mut self) -> Result<(), ParserError> {
        while !self.cancelled {
            if !self.parse_next_frame()? {
                break;
            }
        }
        Ok(())
    }

    fn parse_frame_s1(&mut self) -> Result<bool, ParserError> {
        let cmd = self.bit_reader.read_int(8) as u8;
        let tick = self.bit_reader.read_signed_int(32);
        self.game_state.set_ingame_tick(tick);
        self.bit_reader.read_int(8); // player slot

        match cmd {
            | 3 => Ok(true),  // synctick
            | 7 => Ok(false), // stop
            | 4 | 9 | 8 => {
                let len = self.bit_reader.read_signed_int(32) as u32;
                for _ in 0..len {
                    self.bit_reader.read_int(8);
                }
                Ok(true)
            },
            | 6 => {
                let len = self.bit_reader.read_signed_int(32) as usize;
                let mut data = Vec::with_capacity(len);
                for _ in 0..len {
                    data.push(self.bit_reader.read_int(8) as u8);
                }

                let _ = self.s1_tables.parse_packet(&data);
                self.dispatch_event(crate::events::DataTablesParsed);

                if self.s1_tables.parse_packet(&data).is_ok() {
                    self.server_classes = self.s1_tables.server_classes().to_vec();
                    self.update_equipment_mapping_from_classes();
                    self.dispatch_event(crate::events::DataTablesParsed);
                }

                Ok(true)
            },
            | 5 => {
                self.bit_reader.read_int(32); // unknown
                let len = self.bit_reader.read_signed_int(32) as u32;
                for _ in 0..len {
                    self.bit_reader.read_int(8);
                }
                Ok(true)
            },
            | 1 | 2 => {
                // packet
                const SKIP_BITS: u32 = (152 + 4 + 4) * 8;
                for _ in 0..SKIP_BITS {
                    self.bit_reader.read_bit();
                }
                let size = self.bit_reader.read_signed_int(32) as u32;
                for _ in 0..size {
                    self.bit_reader.read_int(8);
                }
                Ok(true)
            },
            | _ => Ok(true),
        }
        .map(|res| {
            if res {
                self.current_frame += 1;
                self.dispatch_event(crate::events::FrameDone);
            }
            res
        })
    }

    fn parse_frame_s2(&mut self) -> Result<bool, ParserError> {
        let cmd = self.bit_reader.read_varint32();
        let msg_type = cmd & !64;
        let compressed = (cmd & 64) != 0;
        let tick = self.bit_reader.read_varint32();
        self.game_state.set_ingame_tick(tick as i32);
        let size = self.bit_reader.read_varint32();

        let mut buf = Vec::with_capacity(size as usize);
        for _ in 0..size {
            buf.push(self.bit_reader.read_int(8) as u8);
        }

        if compressed {
            buf = snap::raw::Decoder::new()
                .decompress_vec(&buf)
                .map_err(|_| ParserError::UnexpectedEndOfDemo)?;
        }

        // Dispatch a very small subset of messages
        if msg_type == 4 {
            if self.s2_tables.parse_packet(&buf).is_ok() {
                self.dispatch_event(crate::events::DataTablesParsed);
            }
        } else if msg_type == crate::proto::msg::SvcMessages::SvcGameEventList as u32 {
            if let Ok(msg) = crate::proto::msg::all::CsvcMsgGameEventList::decode(&buf[..]) {
                self.on_game_event_list(&msg);
                self.dispatch_net_message(msg);
            }
        } else if msg_type == crate::proto::msg::SvcMessages::SvcPacketEntities as u32 {
            if let Ok(msg) = crate::proto::msg::all::CsvcMsgPacketEntities::decode(&buf[..]) {
                for (ent, op) in self.s2_tables.parse_packet_entities(&msg) {
                    let ev = EntityEvent {
                        entity: ent.clone(),
                        op,
                    };
                    self.dispatch_event(ev.clone());
                    if op.contains(crate::sendtables::EntityOp::CREATED) {
                        self.dispatch_event(EntityCreated { entity: ent });
                    }
                }
            }
        } else if msg_type == crate::proto::msg::SvcMessages::SvcGameEvent as u32 {
            if let Ok(msg) = crate::proto::msg::all::CsvcMsgGameEvent::decode(&buf[..]) {
                self.on_game_event(&msg);
                self.dispatch_net_message(msg);
            }
        }

        let cont = msg_type != 0;
        if cont {
            self.current_frame += 1;
            self.dispatch_event(crate::events::FrameDone);
        }
        Ok(cont)
    }

    pub fn on_game_event_list(&mut self, msg: &crate::proto::msg::all::CsvcMsgGameEventList) {
        self.game_events.handle_game_event_list(msg);
    }

    pub fn on_game_event(&mut self, msg: &crate::proto::msg::all::CsvcMsgGameEvent) {
        let mut handler = std::mem::take(&mut self.game_events);
        handler.handle_game_event(self, msg);
        self.game_events = handler;
    }

    pub fn handle_user_message(&self, um: &crate::proto::msg::all::CsvcMsgUserMessage) {
        use crate::proto::msg::{self as proto_msg};
        if let (Some(t), Some(data)) = (um.msg_type, &um.msg_data) {
            if let Ok(kind) = proto_msg::ECstrike15UserMessages::try_from(t) {
                match kind {
                    | proto_msg::ECstrike15UserMessages::CsUmSayText => {
                        if let Ok(msg) = proto_msg::all::CcsUsrMsgSayText::decode(&data[..]) {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmSayText2 => {
                        if let Ok(msg) = proto_msg::all::CcsUsrMsgSayText2::decode(&data[..]) {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmServerRankUpdate => {
                        if let Ok(msg) =
                            proto_msg::all::CcsUsrMsgServerRankUpdate::decode(&data[..])
                        {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmRoundImpactScoreData => {
                        if let Ok(msg) =
                            proto_msg::all::CcsUsrMsgRoundImpactScoreData::decode(&data[..])
                        {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | _ => {},
                }
            }
        }
    }
}
