#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_packet(&mut self, _data: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}
