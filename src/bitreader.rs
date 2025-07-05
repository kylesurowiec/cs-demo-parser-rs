use bitstream_io::{BitRead, BitReader as StreamBitReader, LittleEndian};
use std::io::{BufReader, Read};

const SMALL_BUFFER: usize = 512;
const LARGE_BUFFER: usize = 1024 * 128;
const MAX_VARINT32_BYTES: u32 = 5;
const MAX_VARINT_BYTES: u32 = 10;

pub struct BitReader<R: Read> {
    inner: StreamBitReader<BufReader<R>, LittleEndian>,
}

impl<R: Read> BitReader<R> {
    fn with_capacity(reader: R, cap: usize) -> Self {
        let buf = BufReader::with_capacity(cap, reader);
        Self {
            inner: StreamBitReader::endian(buf, LittleEndian),
        }
    }

    pub fn new_small(reader: R) -> Self {
        Self::with_capacity(reader, SMALL_BUFFER)
    }

    pub fn new_large(reader: R) -> Self {
        Self::with_capacity(reader, LARGE_BUFFER)
    }

    pub fn read_int(&mut self, bits: u32) -> u32 {
        self.inner.read(bits).unwrap()
    }

    pub fn read_signed_int(&mut self, bits: u32) -> i32 {
        let val = self.read_int(bits);
        if bits == 0 {
            return 0;
        }
        let sign_bit = 1u32 << (bits - 1);
        let mask = if bits >= 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };
        let mut out = val & mask;
        if out & sign_bit != 0 {
            out |= !mask;
        }
        out as i32
    }

    fn read_single_byte(&mut self) -> u8 {
        self.read_int(8) as u8
    }

    pub fn read_string(&mut self) -> String {
        const VALVE_MAX_STRING_LENGTH: usize = 4096;
        self.read_string_limited(VALVE_MAX_STRING_LENGTH, false)
    }

    pub fn read_bit(&mut self) -> bool {
        self.read_int(1) != 0
    }

    pub fn read_float(&mut self) -> f32 {
        f32::from_bits(self.read_int(32))
    }

    fn read_string_limited(&mut self, limit: usize, end_on_newline: bool) -> String {
        let mut result = Vec::with_capacity(256);
        for _ in 0..limit {
            let b = self.read_single_byte();
            if b == 0 || (end_on_newline && b == b'\n') {
                break;
            }
            result.push(b);
        }
        String::from_utf8(result).unwrap_or_default()
    }

    pub fn read_varint32(&mut self) -> u32 {
        let mut res = 0u32;
        let mut b: u32 = 0x80;
        for count in 0..MAX_VARINT32_BYTES {
            if b & 0x80 == 0 {
                break;
            }
            b = self.read_single_byte() as u32;
            res |= (b & 0x7f) << (7 * count);
        }
        res
    }

    pub fn read_signed_varint32(&mut self) -> i32 {
        let res = self.read_varint32();
        ((res >> 1) as i32) ^ -((res & 1) as i32)
    }

    pub fn read_varint64(&mut self) -> u64 {
        let mut res = 0u64;
        let mut b: u64 = 0x80;
        for count in 0..MAX_VARINT_BYTES {
            if b & 0x80 == 0 {
                break;
            }
            b = self.read_single_byte() as u64;
            res |= (b & 0x7f) << (7 * count);
        }
        res
    }

    pub fn read_signed_varint64(&mut self) -> i64 {
        let res = self.read_varint64();
        ((res >> 1) as i64) ^ -((res & 1) as i64)
    }

    pub fn read_ubit_int(&mut self) -> u32 {
        let mut res = self.read_int(6);
        match res & (16 | 32) {
            | 16 => res = (res & 15) | (self.read_int(4) << 4),
            | 32 => res = (res & 15) | (self.read_int(8) << 4),
            | 48 => res = (res & 15) | (self.read_int(32 - 4) << 4),
            | _ => {},
        }
        res
    }

    pub fn read_c_string(&mut self, length: usize) -> String {
        self.read_string_limited(length, false)
    }
}
