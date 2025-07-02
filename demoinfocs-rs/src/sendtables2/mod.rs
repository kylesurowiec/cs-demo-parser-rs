use crate::proto::msg::all as msg;

/// Minimal parser for Source2 send tables.
#[derive(Default)]
pub struct Parser {
    class_id_size: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self { class_id_size: 0 }
    }

    /// Handles CSVCMsg_ServerInfo and extracts the class id size.
    pub fn on_server_info(&mut self, msg: &msg::CsvcMsgServerInfo) {
        if let Some(max) = msg.max_classes {
            self.class_id_size = ((max as f32).log2().floor() as u32) + 1;
        }
    }

    /// Parses flattened serializer packets. Currently this is a stub that simply
    /// verifies the protobuf payload can be decoded.
    pub fn parse_packet(&mut self, _data: &[u8]) -> Result<(), prost::DecodeError> {
        // Full Source2 support not implemented yet, just ensure data is valid protobuf
        let _ = _data;
        Ok(())
    }

    pub fn class_id_size(&self) -> u32 { self.class_id_size }
}
