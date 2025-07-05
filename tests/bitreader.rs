use demoinfocs_rs::bitreader::BitReader;

fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    while value >= 0x80 {
        out.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
    out
}

fn encode_ubit_int(value: u32) -> Vec<u8> {
    let mut bits = Vec::new();
    let mut push = |mut val: u32, n: usize| {
        for _ in 0..n {
            bits.push(val & 1 == 1);
            val >>= 1;
        }
    };

    if value < 16 {
        push(value, 6);
    } else if value < 0x100 {
        push((value & 0xf) | 0x10, 6);
        push(value >> 4, 4);
    } else if value < 0x10000 {
        push((value & 0xf) | 0x20, 6);
        push(value >> 4, 8);
    } else {
        push((value & 0xf) | 0x30, 6);
        push(value >> 4, 28);
    }

    let mut out = Vec::new();
    let mut cur = 0u8;
    let mut count = 0;
    for b in bits {
        if b {
            cur |= 1 << count;
        }
        count += 1;
        if count == 8 {
            out.push(cur);
            cur = 0;
            count = 0;
        }
    }
    if count > 0 {
        out.push(cur);
    }
    out
}

#[test]
fn test_read_string() {
    let data = b"hello\0world" as &[u8];
    let mut r = BitReader::new_small(data);
    assert_eq!(r.read_string(), "hello");
}

#[test]
fn test_varint32() {
    for &val in &[1u32, 300u32, 0xdeadbeefu32] {
        let bytes = encode_varint(val as u64);
        let mut r = BitReader::new_small(&bytes[..]);
        assert_eq!(r.read_varint32(), val);
    }
}

#[test]
fn test_varint64() {
    for &val in &[1u64, 70000u64, 0x12345678abcdefu64] {
        let bytes = encode_varint(val);
        let mut r = BitReader::new_small(&bytes[..]);
        assert_eq!(r.read_varint64(), val);
    }
}

#[test]
fn test_ubit_int() {
    for &val in &[7u32, 31u32, 0x123u32, 0x12345678u32] {
        let bytes = encode_ubit_int(val);
        let mut r = BitReader::new_small(&bytes[..]);
        assert_eq!(r.read_ubit_int(), val);
    }
}
