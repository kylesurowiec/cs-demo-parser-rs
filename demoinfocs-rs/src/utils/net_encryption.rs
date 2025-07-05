use ice_crypt::IceKey;

fn read_varint32(slice: &mut &[u8]) -> u32 {
    let mut res = 0u32;
    let mut shift = 0;
    while shift < 35 && !slice.is_empty() {
        let b = slice[0];
        *slice = &slice[1..];
        res |= ((b & 0x7f) as u32) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    res
}

/// Decrypts an encrypted net-message using the ICE cipher.
///
/// Returns `(msg_type, payload)` on success or `None` if the data is malformed.
pub fn decrypt_message(key: &[u8], data: &[u8]) -> Option<(u32, Vec<u8>)> {
    let mut ice = IceKey::new(2);
    ice.set(key.to_vec());
    let dec = ice.decrypt_all(data.to_vec());
    if dec.len() < 5 {
        return None;
    }
    let padding = dec[0] as usize;
    if padding >= dec.len() - 5 {
        return None;
    }
    let len_start = 1 + padding;
    let n_bytes_written = u32::from_be_bytes([
        dec[len_start],
        dec[len_start + 1],
        dec[len_start + 2],
        dec[len_start + 3],
    ]) as usize;
    if dec.len() != 1 + padding + 4 + n_bytes_written {
        return None;
    }
    let mut slice = &dec[len_start + 4..];
    let msg_type = read_varint32(&mut slice);
    let size = read_varint32(&mut slice) as usize;
    if slice.len() < size {
        return None;
    }
    let payload = slice[..size].to_vec();
    Some((msg_type, payload))
}
