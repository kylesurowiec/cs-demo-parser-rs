use crate::bitreader::BitReader;

#[derive(Debug, Default)]
pub struct LumpInfo {
    pub data_size: u64,
}

impl LumpInfo {
    pub fn parse<R: std::io::Read>(reader: &mut BitReader<R>) -> Option<Self> {
        // Peek the next 4 bytes to check for the expected magic.
        let magic = reader.read_int(32) as u32;
        if magic != 0xba80b001 {
            // Not a lump table, rewind.
            return None;
        }
        let count = reader.read_int(32) as u32;
        // Skip two unknown fields
        reader.read_int(32);
        reader.read_int(32);
        let mut max_end = 0u64;
        for _ in 0..count {
            let mut vals = [0u32; 8];
            for v in &mut vals {
                *v = reader.read_int(32) as u32;
            }
            for pair in (0..8).step_by(2) {
                let end = vals[pair] as u64 + vals[pair + 1] as u64;
                if end > max_end {
                    max_end = end;
                }
            }
        }
        Some(Self { data_size: max_end })
    }
}
