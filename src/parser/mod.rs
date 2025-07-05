use crate::bitreader::BitReader;
use crate::dispatcher::{Dispatcher, EventDispatcher, HandlerIdentifier};
use crate::game_state::GameState;
use crate::sendtables1::TablesParser;
use crate::sendtables2;
use crate::stringtables;

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

#[derive(Clone, Debug)]
pub struct StringTableUpdated {
    pub table: crate::stringtables::StringTable,
}

/// Configuration options for [`Parser`].
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ParserConfig {
    /// Size of the internal message queue. `None` uses the default buffer
    /// size which is automatically determined from the demo header.
    pub msg_queue_size: Option<usize>,

    /// Decryption key for encrypted net-messages.
    pub decryption_key: Option<Vec<u8>>,

    /// Ignore malformed encrypted net-messages.
    pub ignore_bad_encrypted_data: bool,

    /// Ignore encrypted net-message warnings when no decryption key is set.
    pub ignore_missing_decryption_key: bool,

    /// Ignore errors about missing bombsite indices in game events.
    pub ignore_bombsite_index_not_found: bool,

    /// Disable mimicking Source 1 game events when parsing Source 2 demos.
    pub disable_mimic_source1_events: bool,

    /// Fallback protobuf for game event lists in Source 2 demos.
    pub source2_fallback_game_event_list_bin: Option<Vec<u8>>,

    /// Ignore PacketEntities parsing panics.
    pub ignore_packet_entities_panic: bool,

    /// Override the tick rate in Hz when no information is available in the demo.
    pub tick_rate_override: Option<f64>,
}


/// Parser for CS:GO / CS2 demo files.
pub struct Parser<R: Read> {
    bit_reader: BitReader<R>,
    event_dispatcher: Arc<EventDispatcher>,
    msg_dispatcher: Arc<EventDispatcher>,
    user_msg_dispatcher: Arc<EventDispatcher>,
    s2_tables: sendtables2::Parser,
    s1_tables: TablesParser,
    string_tables: stringtables::StringTables,
    server_classes: Vec<crate::sendtables::ServerClass>,
    equipment_mapping: HashMap<String, crate::common::EquipmentType>,
    game_state: GameState,
    current_frame: i32,
    cancelled: bool,
    game_events: crate::game_events::GameEventHandler,
    header: Option<DemoHeader>,
    config: ParserConfig,
}

impl<R: Read> Parser<R> {
    /// Creates a new [`Parser`] from the given reader using [`ParserConfig::default`].
    pub fn new(reader: R) -> Self {
        Self::with_config(reader, ParserConfig::default())
    }

    /// Creates a new [`Parser`] from the given reader and configuration.
    pub fn with_config(reader: R, config: ParserConfig) -> Self {
        Self {
            bit_reader: BitReader::new_large(reader),
            event_dispatcher: EventDispatcher::with_capacity(config.msg_queue_size),
            msg_dispatcher: EventDispatcher::with_capacity(config.msg_queue_size),
            user_msg_dispatcher: EventDispatcher::with_capacity(config.msg_queue_size),
            s2_tables: sendtables2::Parser::new(),
            s1_tables: TablesParser::new(),
            string_tables: stringtables::StringTables::new(),
            server_classes: Vec::new(),
            equipment_mapping: HashMap::new(),
            game_state: GameState::default(),
            current_frame: 0,
            cancelled: false,
            game_events: crate::game_events::GameEventHandler::new(),
            header: None,
            config,
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

    pub fn register_on_string_table<F>(&self, handler: F) -> HandlerIdentifier
    where
        F: Fn(&crate::stringtables::StringTable) + Send + Sync + 'static,
    {
        self.event_dispatcher
            .register_handler::<StringTableUpdated, _>(move |ev| handler(&ev.table))
    }

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn string_table(&self, name: &str) -> Option<&crate::stringtables::StringTable> {
        self.string_tables.get(name)
    }

    fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    fn update_equipment_mapping_from_classes(&mut self) {
        let item_defs = self.string_tables.get("ItemDefinitions");
        self.equipment_mapping =
            datatable::build_equipment_mapping(&self.server_classes, item_defs);
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
        if let Some(rate) = self.config.tick_rate_override {
            return rate;
        }
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
        if let Some(h) = &self.header
            && h.playback_frames > 0 {
                return self.current_frame as f32 / h.playback_frames as f32;
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
            | 4 | 9 => {
                let len = self.bit_reader.read_signed_int(32) as u32;
                for _ in 0..len {
                    self.bit_reader.read_int(8);
                }
                Ok(true)
            },
            | 8 => {
                let len = self.bit_reader.read_signed_int(32) as usize;
                let mut data = Vec::with_capacity(len);
                for _ in 0..len {
                    data.push(self.bit_reader.read_int(8) as u8);
                }
                self.parse_stringtable_packet(&data);
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
        .inspect(|&res| {
            if res {
                self.current_frame += 1;
                self.dispatch_event(crate::events::FrameDone);
            }
        })
    }

    fn parse_stringtable_packet(&mut self, data: &[u8]) {
        let updates = self.string_tables.parse_packet(data);
        for t in updates {
            self.dispatch_event(StringTableUpdated { table: t });
        }
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

        if msg_type == 4 {
            if self.s2_tables.parse_packet(&buf).is_ok() {
                self.dispatch_event(crate::events::DataTablesParsed);
            }
        } else {
            self.handle_svc_message(msg_type, &buf);
        }

        let cont = msg_type != 0;
        if cont {
            self.current_frame += 1;
            self.dispatch_event(crate::events::FrameDone);
        }
        Ok(cont)
    }

    pub fn on_game_event_list(&mut self, msg: &proto_msg::CsvcMsgGameEventList) {
        self.game_events.handle_game_event_list(msg);
    }

    pub fn on_game_event(&mut self, msg: &proto_msg::CsvcMsgGameEvent) {
        let handler = std::mem::take(&mut self.game_events);
        handler.handle_game_event(self, msg);
        self.game_events = handler;
    }

    pub fn handle_user_message(&mut self, um: &proto_msg::CsvcMsgUserMessage) {
        use crate::proto::msg::{self as proto_msg};
        if let (Some(t), Some(data)) = (um.msg_type, &um.msg_data)
            && let Ok(kind) = proto_msg::ECstrike15UserMessages::try_from(t) {
                match kind {
                    | proto_msg::ECstrike15UserMessages::CsUmSayText => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgSayText::decode(&data[..]) {
                            self.dispatch_user_message(msg.clone());
                            self.dispatch_event(crate::events::ChatMessage {
                                sender: None,
                                text: msg.text.clone().unwrap_or_default(),
                                is_chat_all: msg.textallchat.unwrap_or_default(),
                            });
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmVguiMenu => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgVguiMenu::decode(&data[..]) {
                            let keys = msg
                                .subkeys
                                .iter()
                                .map(|k| {
                                    (
                                        k.name.clone().unwrap_or_default(),
                                        k.str.clone().unwrap_or_default(),
                                    )
                                })
                                .collect();
                            self.dispatch_user_message(crate::events::VguiMenu {
                                name: msg.name.unwrap_or_default(),
                                show: msg.show.unwrap_or(false),
                                keys,
                            });
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmSayText2 => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgSayText2::decode(&data[..]) {
                            self.dispatch_user_message(msg.clone());
                            let text = msg.params.get(1).cloned().unwrap_or_default();
                            self.dispatch_event(crate::events::ChatMessage {
                                sender: None,
                                text,
                                is_chat_all: msg.textallchat.unwrap_or_default(),
                            });
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmServerRankUpdate => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgServerRankUpdate::decode(&data[..]) {
                            for ru in &msg.rank_update {
                                self.dispatch_event(crate::events::RankUpdate {
                                    steam_id32: ru.account_id.unwrap_or_default(),
                                    rank_change: ru.rank_change.unwrap_or_default(),
                                    rank_old: ru.rank_old.unwrap_or_default(),
                                    rank_new: ru.rank_new.unwrap_or_default(),
                                    win_count: ru.num_wins.unwrap_or_default(),
                                    player: None,
                                });
                            }
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmRoundBackupFilenames => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgRoundBackupFilenames::decode(&data[..])
                        {
                            self.dispatch_user_message(crate::events::RoundBackupFilenames {
                                count: msg.count.unwrap_or_default(),
                                index: msg.index.unwrap_or_default(),
                                filename: msg.filename.unwrap_or_default(),
                                nicename: msg.nicename.unwrap_or_default(),
                            });
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmRoundImpactScoreData => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgRoundImpactScoreData::decode(&data[..])
                        {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmTextMsg => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgTextMsg::decode(&data[..]) {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmHintText => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgHintText::decode(&data[..]) {
                            self.dispatch_user_message(msg);
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmShowMenu => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgShowMenu::decode(&data[..]) {
                            self.dispatch_user_message(crate::events::ShowMenu {
                                bits_valid_slots: msg.bits_valid_slots.unwrap_or_default(),
                                display_time: msg.display_time.unwrap_or_default(),
                                menu_string: msg.menu_string.unwrap_or_default(),
                            });
                        }
                    },
                    | proto_msg::ECstrike15UserMessages::CsUmBarTime => {
                        if let Ok(msg) = proto_msg::CcsUsrMsgBarTime::decode(&data[..]) {
                            self.dispatch_user_message(crate::events::BarTime {
                                time: msg.time.unwrap_or_default(),
                            });
                        }
                    },
                    | _ => {},
                }
            }
    }

    fn handle_encrypted_data(&mut self, msg: &proto_msg::CsvcMsgEncryptedData) {
        if msg.key_type != Some(2) {
            return;
        }
        match (&self.config.decryption_key, msg.encrypted.as_deref()) {
            | (Some(key), Some(enc)) => {
                if let Some((cmd, payload)) =
                    crate::utils::net_encryption::decrypt_message(key, enc)
                {
                    self.handle_svc_message(cmd, &payload);
                } else if !self.config.ignore_bad_encrypted_data {
                    self.dispatch_event(crate::events::ParserWarn {
                        message: "encrypted net-message has invalid length".into(),
                        r#type: crate::events::WarnType::CantReadEncryptedNetMessage,
                    });
                }
            },
            | (None, Some(_)) => {
                if !self.config.ignore_missing_decryption_key {
                    self.dispatch_event(crate::events::ParserWarn {
                        message: "received encrypted net-message but no decryption key is set"
                            .into(),
                        r#type: crate::events::WarnType::MissingNetMessageDecryptionKey,
                    });
                }
            },
            | _ => {},
        }
    }

    fn handle_svc_message(&mut self, msg_type: u32, buf: &[u8]) {
        if let Ok(kind) = proto_msg::SvcMessages::try_from(msg_type as i32) {
            match kind {
                | proto_msg::SvcMessages::SvcServerInfo => {
                    if let Ok(msg) = proto_msg::CsvcMsgServerInfo::decode(buf) {
                        self.s2_tables.on_server_info(&msg);
                        self.game_state.match_info.map = msg.map_name.clone();
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcSendTable => {
                    if let Ok(msg) = proto_msg::CsvcMsgSendTable::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcClassInfo => {
                    if let Ok(msg) = proto_msg::CsvcMsgClassInfo::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcSetPause => {
                    if let Ok(msg) = proto_msg::CsvcMsgSetPause::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcCreateStringTable => {
                    if let Ok(msg) = proto_msg::CsvcMsgCreateStringTable::decode(buf) {
                        if let Some(t) = self.string_tables.on_create_string_table(&msg) {
                            self.dispatch_event(crate::events::StringTableCreated {
                                table_name: t.name.clone(),
                            });
                            self.dispatch_event(StringTableUpdated { table: t });
                        }
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcUpdateStringTable => {
                    if let Ok(msg) = proto_msg::CsvcMsgUpdateStringTable::decode(buf) {
                        if let Some(t) = self.string_tables.on_update_string_table(&msg) {
                            self.dispatch_event(StringTableUpdated { table: t });
                        }
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcVoiceInit => {
                    if let Ok(msg) = proto_msg::CsvcMsgVoiceInit::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcVoiceData => {
                    if let Ok(msg) = proto_msg::CsvcMsgVoiceData::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcPrint => {
                    if let Ok(msg) = proto_msg::CsvcMsgPrint::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcSounds => {
                    if let Ok(msg) = proto_msg::CsvcMsgSounds::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcSetView => {
                    if let Ok(msg) = proto_msg::CsvcMsgSetView::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcFixAngle => {
                    if let Ok(msg) = proto_msg::CsvcMsgFixAngle::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcCrosshairAngle => {
                    if let Ok(msg) = proto_msg::CsvcMsgCrosshairAngle::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcBspDecal => {
                    if let Ok(msg) = proto_msg::CsvcMsgBspDecal::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcSplitScreen => {
                    if let Ok(msg) = proto_msg::CsvcMsgSplitScreen::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcUserMessage => {
                    if let Ok(msg) = proto_msg::CsvcMsgUserMessage::decode(buf) {
                        self.handle_user_message(&msg);
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcEntityMessage => {
                    if let Ok(msg) = proto_msg::CsvcMsgEntityMsg::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcGameEvent => {
                    if let Ok(msg) = proto_msg::CsvcMsgGameEvent::decode(buf) {
                        self.on_game_event(&msg);
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcPacketEntities => {
                    if let Ok(msg) = proto_msg::CsvcMsgPacketEntities::decode(buf) {
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
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcTempEntities => {
                    if let Ok(msg) = proto_msg::CsvcMsgTempEntities::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcPrefetch => {
                    if let Ok(msg) = proto_msg::CsvcMsgPrefetch::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcMenu => {
                    if let Ok(msg) = proto_msg::CsvcMsgMenu::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcGameEventList => {
                    if let Ok(msg) = proto_msg::CsvcMsgGameEventList::decode(buf) {
                        self.on_game_event_list(&msg);
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcGetCvarValue => {
                    if let Ok(msg) = proto_msg::CsvcMsgGetCvarValue::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcPaintmapData => {
                    if let Ok(msg) = proto_msg::CsvcMsgPaintmapData::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcCmdKeyValues => {
                    if let Ok(msg) = proto_msg::CsvcMsgCmdKeyValues::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcEncryptedData => {
                    if let Ok(msg) = proto_msg::CsvcMsgEncryptedData::decode(buf) {
                        self.handle_encrypted_data(&msg);
                    }
                },
                | proto_msg::SvcMessages::SvcHltvReplay => {
                    if let Ok(msg) = proto_msg::CsvcMsgHltvReplay::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::SvcMessages::SvcBroadcastCommand => {
                    if let Ok(msg) = proto_msg::CsvcMsgBroadcastCommand::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
            }
        } else if let Ok(net) = proto_msg::NetMessages::try_from(msg_type as i32) {
            match net {
                | proto_msg::NetMessages::NetNop => {
                    if let Ok(msg) = proto_msg::CnetMsgNop::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetDisconnect => {
                    if let Ok(msg) = proto_msg::CnetMsgDisconnect::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetFile => {
                    if let Ok(msg) = proto_msg::CnetMsgFile::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetSplitScreenUser => {
                    if let Ok(msg) = proto_msg::CnetMsgSplitScreenUser::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetTick => {
                    if let Ok(msg) = proto_msg::CnetMsgTick::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetStringCmd => {
                    if let Ok(msg) = proto_msg::CnetMsgStringCmd::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetSetConVar => {
                    if let Ok(msg) = proto_msg::CnetMsgSetConVar::decode(buf) {
                        if let Some(ref cvars) = msg.convars {
                            let mut map = HashMap::new();
                            for cv in &cvars.cvars {
                                if let (Some(name), Some(value)) =
                                    (cv.name.clone(), cv.value.clone())
                                {
                                    map.insert(name, value);
                                }
                            }
                            if !map.is_empty() {
                                self.dispatch_event(crate::events::ConVarsUpdated {
                                    updated_con_vars: map.clone(),
                                });
                            }
                        }
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetSignonState => {
                    if let Ok(msg) = proto_msg::CnetMsgSignonState::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
                | proto_msg::NetMessages::NetPlayerAvatarData => {
                    if let Ok(msg) = proto_msg::CnetMsgPlayerAvatarData::decode(buf) {
                        self.dispatch_net_message(msg);
                    }
                },
            }
        }
    }
}

use crate::proto::msg::cs_demo_parser_rs as proto_msg;
