use std::io::{Read, Seek, SeekFrom};
pub use crate::parser::DemoHeader;

pub fn parse_demo_header<R: Read + Seek>(reader: &mut R) -> Result<DemoHeader, std::io::Error> {
    // Read the entire header as raw bytes first
    let mut header_bytes = vec![0u8; 1072];
    reader.read_exact(&mut header_bytes)?;
    
    let mut header = DemoHeader::default();
    
    // File stamp (8 bytes)
    let null_pos = header_bytes[0..8].iter().position(|&x| x == 0).unwrap_or(8);
    header.filestamp = String::from_utf8_lossy(&header_bytes[0..null_pos]).into_owned();
    
    // Protocol (4 bytes, offset 8)
    header.protocol = i32::from_le_bytes([
        header_bytes[8], header_bytes[9], header_bytes[10], header_bytes[11]
    ]);
    
    // Network protocol (4 bytes, offset 12)
    header.network_protocol = i32::from_le_bytes([
        header_bytes[12], header_bytes[13], header_bytes[14], header_bytes[15]
    ]);
    
    // Server name (260 bytes, offset 16)
    let server_end = header_bytes[16..276].iter().position(|&x| x == 0).unwrap_or(260) + 16;
    header.server_name = String::from_utf8_lossy(&header_bytes[16..server_end]).into_owned();
    
    // Client name (260 bytes, offset 276)
    let client_end = header_bytes[276..536].iter().position(|&x| x == 0).unwrap_or(260) + 276;
    header.client_name = String::from_utf8_lossy(&header_bytes[276..client_end]).into_owned();
    
    // Map name (260 bytes, offset 536)
    let map_end = header_bytes[536..796].iter().position(|&x| x == 0).unwrap_or(260) + 536;
    header.map_name = String::from_utf8_lossy(&header_bytes[536..map_end]).into_owned();
    
    // Game directory (260 bytes, offset 796)
    let game_dir_end = header_bytes[796..1056].iter().position(|&x| x == 0).unwrap_or(260) + 796;
    header.game_directory = String::from_utf8_lossy(&header_bytes[796..game_dir_end]).into_owned();
    
    // Playback time (4 bytes float, offset 1056)
    header.playback_time = f32::from_le_bytes([
        header_bytes[1056], header_bytes[1057], header_bytes[1058], header_bytes[1059]
    ]);
    
    // Playback ticks (4 bytes int, offset 1060)
    header.playback_ticks = i32::from_le_bytes([
        header_bytes[1060], header_bytes[1061], header_bytes[1062], header_bytes[1063]
    ]);
    
    // Playback frames (4 bytes int, offset 1064)
    header.playback_frames = i32::from_le_bytes([
        header_bytes[1064], header_bytes[1065], header_bytes[1066], header_bytes[1067]
    ]);
    
    // Signon length (4 bytes int, offset 1068)
    header.signon_length = i32::from_le_bytes([
        header_bytes[1068], header_bytes[1069], header_bytes[1070], header_bytes[1071]
    ]);
    
    Ok(header)
}

// Alternative BitReader fix - replace the read_c_string method in BitReader
impl<R: Read> crate::bitreader::BitReader<R> {
    /// Fixed version that properly handles fixed-width string fields
    pub fn read_c_string_fixed(&mut self, field_size: usize) -> String {
        let mut result = Vec::new();
        
        for _ in 0..field_size {
            let byte = self.read_int(8) as u8;
            if byte == 0 {
                // Found null terminator, but we still need to consume the rest of the field
                for _ in result.len() + 1..field_size {
                    self.read_int(8); // consume remaining bytes
                }
                break;
            }
            result.push(byte);
        }
        
        String::from_utf8(result).unwrap_or_default()
    }
}

// Update to parser.rs - modify the parse_header method
impl<R: Read + Seek> crate::parser::Parser<R> {
    /// Updated header parsing that uses the fixed implementation
    pub fn parse_header_fixed(&mut self) -> Result<DemoHeader, crate::parser::ParserError> {
        // Reset to beginning of file
        if let Err(_) = self.bit_reader.get_inner_mut().seek(SeekFrom::Start(0)) {
            return Err(crate::parser::ParserError::UnexpectedEndOfDemo);
        }
        
        // Use the new fixed header parser
        let header = parse_demo_header(self.bit_reader.get_inner_mut())
            .map_err(|_| crate::parser::ParserError::UnexpectedEndOfDemo)?;
        
        // Validate the header
        if !header.filestamp.starts_with("HL2DEMO") && !header.filestamp.starts_with("PBDEMS2") {
            return Err(crate::parser::ParserError::InvalidFileType);
        }
        
        self.header = Some(header.clone());
        Ok(header)
    }
}