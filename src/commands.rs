// Utilities for crafting Source 2 demo commands
use bitstream_io::{BitWrite, BitWriter, LittleEndian};
use prost::Message;

use crate::proto::msgs2::{CDemoPacket, NetMessages};

fn write_ubit_int<W: std::io::Write>(
    writer: &mut BitWriter<W, LittleEndian>,
    value: u32,
) -> std::io::Result<()> {
    let lower = value & 0xF;
    let high = value >> 4;
    if high == 0 {
        writer.write(6, lower)?;
    } else if high < (1 << 4) {
        writer.write(6, lower | 0x10)?;
        writer.write(4, high)?;
    } else if high < (1 << 8) {
        writer.write(6, lower | 0x20)?;
        writer.write(8, high)?;
    } else {
        writer.write(6, lower | 0x30)?;
        writer.write(28, high)?;
    }
    Ok(())
}

fn write_varint32<W: std::io::Write>(
    writer: &mut BitWriter<W, LittleEndian>,
    mut value: u32,
) -> std::io::Result<()> {
    loop {
        let mut b = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            b |= 0x80;
        }
        writer.write(8, b as u32)?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

/// Builder for creating [`CDemoPacket`] messages from net messages.
pub struct CommandBuilder {
    writer: BitWriter<Vec<u8>, LittleEndian>,
}

impl CommandBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            writer: BitWriter::endian(Vec::new(), LittleEndian),
        }
    }

    /// Append a Source 2 net message.
    pub fn push_net_message<M: Message>(
        &mut self,
        ty: NetMessages,
        msg: &M,
    ) -> std::io::Result<()> {
        self.push_raw_net_message(ty, &msg.encode_to_vec())
    }

    /// Append a raw encoded net message.
    pub fn push_raw_net_message(&mut self, ty: NetMessages, bytes: &[u8]) -> std::io::Result<()> {
        write_ubit_int(&mut self.writer, ty as u32)?;
        write_varint32(&mut self.writer, bytes.len() as u32)?;
        for b in bytes {
            self.writer.write(8, *b as u32)?;
        }
        Ok(())
    }

    /// Finish building and return the [`CDemoPacket`].
    pub fn into_packet(mut self) -> CDemoPacket {
        let _ = self.writer.byte_align();
        CDemoPacket {
            data: Some(self.writer.into_writer()),
        }
    }

    /// Finish building and return the encoded bytes without wrapping in a [`CDemoPacket`].
    pub fn into_bytes(mut self) -> Vec<u8> {
        let _ = self.writer.byte_align();
        self.writer.into_writer()
    }
}

impl Default for CommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}
