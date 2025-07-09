use crate::bitreader::BitReader;

/// Magic identifying a lump table following the demo header.
pub const LUMP_MAGIC: u32 = 0xba80b001;

/// Information about lump data found in a demo.
#[derive(Debug, Default)]
pub struct LumpInfo {
    /// The total size of all lump data following the lump table.
    pub data_size: u64,
}

impl LumpInfo {
    /// Parses the lump table that follows the demo header and returns the
    /// combined size of all lumps. The reader is left positioned at the first
    /// byte after the table.
    pub fn parse<R: std::io::Read>(reader: &mut BitReader<R>) -> Self {
        // Magic identifying a lump table.

        let magic = reader.read_int(32);
        debug_assert_eq!(magic, LUMP_MAGIC, "unexpected lump table magic");

        let count = reader.read_int(32);
        // Skip two unknown fields
        reader.read_int(32);
        reader.read_int(32);

        let mut max_end = 0u64;
        for _ in 0..count {
            let offset = reader.read_int(32) as u64;
            let length = reader.read_int(32) as u64;
            // version
            reader.read_int(32);
            // fourcc / flags
            reader.read_int(32);

            let end = offset + length;
            if end > max_end {
                max_end = end;
            }
        }

        Self { data_size: max_end }
    }
}
