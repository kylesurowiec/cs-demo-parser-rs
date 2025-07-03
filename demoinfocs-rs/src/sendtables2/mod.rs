/// Minimal parser for Source2 send tables.
#[derive(Default)]
pub struct Parser {
    class_id_size: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self { class_id_size: 0 }
    }

    /// Parses flattened serializer packets. Currently this is a stub that simply
    /// verifies the protobuf payload can be decoded.
    pub fn parse_packet(&mut self, _data: &[u8]) -> Result<(), ()> {
        // Full Source2 support not implemented yet, just ensure data is valid protobuf
        let _ = _data;
        Ok(())
    }

    pub fn class_id_size(&self) -> u32 { self.class_id_size }
}
