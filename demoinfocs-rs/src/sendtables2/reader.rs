pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    bit_val: u64,
    bit_count: u32,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            bit_val: 0,
            bit_count: 0,
        }
    }

    fn next_byte(&mut self) -> u8 {
        let b = self.buf.get(self.pos).copied().unwrap_or(0);
        self.pos += 1;
        b
    }

    pub fn read_bits(&mut self, n: u32) -> u32 {
        while n > self.bit_count {
            self.bit_val |= (self.next_byte() as u64) << self.bit_count;
            self.bit_count += 8;
        }
        let mask = if n == 32 { u64::MAX } else { (1u64 << n) - 1 };
        let x = (self.bit_val & mask) as u32;
        self.bit_val >>= n;
        self.bit_count -= n;
        x
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.bit_count == 0 {
            self.next_byte()
        } else {
            self.read_bits(8) as u8
        }
    }

    pub fn read_var_uint32(&mut self) -> u32 {
        let mut x = 0u32;
        let mut s = 0u32;
        for _ in 0..5 {
            let b = self.read_byte() as u32;
            x |= (b & 0x7f) << s;
            if b & 0x80 == 0 {
                break;
            }
            s += 7;
        }
        x
    }

    pub fn read_ubit_var(&mut self) -> u32 {
        let mut ret = self.read_bits(6);
        match ret & 0x30 {
            | 16 => {
                ret = (ret & 15) | (self.read_bits(4) << 4);
            },
            | 32 => {
                ret = (ret & 15) | (self.read_bits(8) << 4);
            },
            | 48 => {
                ret = (ret & 15) | (self.read_bits(28) << 4);
            },
            | _ => {},
        }
        ret
    }
}
