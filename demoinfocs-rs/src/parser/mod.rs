use crate::bitreader::BitReader;
use crate::dispatcher::{Dispatcher, EventDispatcher, HandlerIdentifier};
use crate::sendtables2;
use crate::events;
use prost::Message;
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

/// Parser for CS:GO / CS2 demo files.
pub struct Parser<R: Read> {
    bit_reader: BitReader<R>,
    event_dispatcher: Arc<EventDispatcher>,
    msg_dispatcher: Arc<EventDispatcher>,
    s2_tables: sendtables2::Parser,
    header: Option<DemoHeader>,
}

impl<R: Read> Parser<R> {
    /// Creates a new [`Parser`] from the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            bit_reader: BitReader::new_large(reader),
            event_dispatcher: EventDispatcher::new(),
            msg_dispatcher: EventDispatcher::new(),
            s2_tables: sendtables2::Parser::new(),
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

    pub fn dispatch_event<E>(&self, event: E)
    where
        E: Send + Sync + 'static,
    {
        self.event_dispatcher.dispatch(event);
    }

    pub fn dispatch_net_message<M>(&self, msg: M)
    where
        M: Send + Sync + 'static,
    {
        self.msg_dispatcher.dispatch(msg);
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
        while self.parse_next_frame()? {}
        Ok(())
    }

    fn parse_frame_s1(&mut self) -> Result<bool, ParserError> {
        let cmd = self.bit_reader.read_int(8) as u8;
        let _tick = self.bit_reader.read_signed_int(32);
        self.bit_reader.read_int(8); // player slot

        match cmd {
            | 3 => Ok(true),  // synctick
            | 7 => Ok(false), // stop
            | 4 | 6 | 9 | 8 => {
                let len = self.bit_reader.read_signed_int(32) as u32;
                for _ in 0..len {
                    self.bit_reader.read_int(8);
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
            if res { self.dispatch_event(crate::events::FrameDone); }
            res
        })
    }

    fn parse_frame_s2(&mut self) -> Result<bool, ParserError> {
        let cmd = self.bit_reader.read_varint32();
        let msg_type = cmd & !64;
        let compressed = (cmd & 64) != 0;
        let _tick = self.bit_reader.read_varint32();
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
        if msg_type == crate::proto::msg::SvcMessages::SvcServerInfo as u32 {
            if let Ok(msg) = crate::proto::msg::all::CsvcMsgServerInfo::decode(&buf[..]) {
                self.s2_tables.on_server_info(&msg);
                self.dispatch_net_message(msg);
            }
        }

        let cont = msg_type != 0;
        if cont { self.dispatch_event(crate::events::FrameDone); }
        Ok(cont)
    }
}
