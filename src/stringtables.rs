use std::collections::HashMap;

use crate::proto::msg::cs_demo_parser_rs as msg;
use prost::Message;

#[derive(Debug, Default, Clone)]
pub struct StringTableEntry {
    pub value: String,
    pub user_data: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct StringTable {
    pub name: String,
    pub entries: HashMap<i32, StringTableEntry>,
    pub max_entries: i32,
    pub user_data_fixed_size: bool,
    pub user_data_size_bits: i32,
}

#[derive(Default)]
pub struct StringTables {
    tables: HashMap<i32, StringTable>,
    name_to_id: HashMap<String, i32>,
}

impl StringTables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<&StringTable> {
        self.name_to_id.get(name).and_then(|id| self.tables.get(id))
    }

    pub fn on_create_string_table(
        &mut self,
        msg: &msg::CsvcMsgCreateStringTable,
    ) -> Option<StringTable> {
        let id = self.tables.len() as i32;
        let mut table = StringTable {
            name: msg.name.clone().unwrap_or_default(),
            max_entries: msg.max_entries.unwrap_or_default(),
            user_data_fixed_size: msg.user_data_fixed_size.unwrap_or(false),
            user_data_size_bits: msg.user_data_size_bits.unwrap_or(0),
            ..Default::default()
        };
        if let (Some(num), Some(data)) = (msg.num_entries, &msg.string_data) {
            parse_s1_entries(&mut table, num, data);
        }
        self.name_to_id.insert(table.name.clone(), id);
        self.tables.insert(id, table.clone());
        Some(table)
    }

    pub fn on_update_string_table(
        &mut self,
        msg: &msg::CsvcMsgUpdateStringTable,
    ) -> Option<StringTable> {
        if let Some(id) = msg.table_id {
            if let Some(table) = self.tables.get_mut(&id) {
                if let (Some(num), Some(data)) = (msg.num_changed_entries, &msg.string_data) {
                    parse_s1_entries_update(table, num, data);
                }
                return Some(table.clone());
            }
        }
        None
    }

    pub fn parse_svc_message(&mut self, typ: msg::SvcMessages, data: &[u8]) -> Option<StringTable> {
        match typ {
            | msg::SvcMessages::SvcCreateStringTable => msg::CsvcMsgCreateStringTable::decode(data)
                .ok()
                .and_then(|m| self.on_create_string_table(&m)),
            | msg::SvcMessages::SvcUpdateStringTable => msg::CsvcMsgUpdateStringTable::decode(data)
                .ok()
                .and_then(|m| self.on_update_string_table(&m)),
            | _ => None,
        }
    }
}

fn read_var_uint32(slice: &mut &[u8]) -> u32 {
    let mut x = 0u32;
    let mut s = 0u32;
    for _ in 0..5 {
        if slice.is_empty() {
            break;
        }
        let b = slice[0];
        *slice = &slice[1..];
        x |= ((b & 0x7f) as u32) << s;
        if b & 0x80 == 0 {
            break;
        }
        s += 7;
    }
    x
}

fn parse_s1_entries(table: &mut StringTable, num: i32, data: &[u8]) {
    use std::io::Cursor;
    let mut r = crate::bitreader::BitReader::new_small(Cursor::new(data));
    if r.read_bit() {
        return;
    }
    let bits = if table.max_entries > 0 {
        ((table.max_entries as f32).log2().ceil()) as u32
    } else {
        0
    };
    let mut idx: i32 = -1;
    let mut history: Vec<String> = Vec::new();
    for _ in 0..num {
        if r.read_bit() {
            idx += 1;
        } else {
            idx = r.read_int(bits) as i32;
        }
        let mut key = String::new();
        if r.read_bit() {
            if r.read_bit() {
                let hist_idx = r.read_int(5) as usize;
                let bytes = r.read_int(5) as usize;
                if hist_idx < history.len() {
                    let h = &history[hist_idx];
                    let slice = &h[..bytes.min(h.len())];
                    key.push_str(slice);
                }
                key.push_str(&r.read_string());
            } else {
                key = r.read_string();
            }
        }
        if history.len() >= 32 {
            history.remove(0);
        }
        history.push(key.clone());
        let mut user_data = Vec::new();
        if r.read_bit() {
            if table.user_data_fixed_size {
                let bytes = (table.user_data_size_bits as u32 + 7) / 8;
                for _ in 0..bytes {
                    user_data.push(r.read_int(8) as u8);
                }
            } else {
                let len = r.read_int(14) as usize;
                for _ in 0..len {
                    user_data.push(r.read_int(8) as u8);
                }
            }
        }
        table.entries.insert(idx, StringTableEntry { value: key, user_data });
    }
}

fn parse_s1_entries_update(table: &mut StringTable, num: i32, data: &[u8]) {
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        parse_s1_entries(table, num, data);
    }));
    if res.is_err() {
        let entry = StringTableEntry {
            value: String::from_utf8_lossy(data).into_owned(),
            user_data: data.to_vec(),
        };
        let idx = table.entries.len() as i32;
        table.entries.insert(idx, entry);
    }
}

impl StringTables {
    pub fn parse_packet(&mut self, data: &[u8]) -> Vec<StringTable> {
        let mut updates = Vec::new();
        let mut slice = data;
        while !slice.is_empty() {
            let msg_id = read_var_uint32(&mut slice);
            let size = read_var_uint32(&mut slice) as usize;
            if slice.len() < size {
                break;
            }
            let (msg_buf, rest) = slice.split_at(size);
            slice = rest;
            if let Ok(t) = msg::SvcMessages::try_from(msg_id as i32) {
                if let Some(tbl) = self.parse_svc_message(t, msg_buf) {
                    updates.push(tbl);
                }
            }
        }
        updates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_var(buf: &mut Vec<u8>, mut value: u32) {
        while value >= 0x80 {
            buf.push(((value & 0x7f) as u8) | 0x80);
            value >>= 7;
        }
        buf.push(value as u8);
    }

    #[test]
    fn test_parse_and_lookup() {
        let mut tables = StringTables::new();

        let mut create = msg::CsvcMsgCreateStringTable::default();
        create.name = Some("test".into());
        let mut payload = Vec::new();
        create.encode(&mut payload).unwrap();
        let mut packet = Vec::new();
        write_var(&mut packet, msg::SvcMessages::SvcCreateStringTable as u32);
        write_var(&mut packet, payload.len() as u32);
        packet.extend_from_slice(&payload);

        let updates = tables.parse_packet(&packet);
        assert_eq!(1, updates.len());
        assert!(tables.get("test").is_some());

        let mut update = msg::CsvcMsgUpdateStringTable::default();
        update.table_id = Some(0);
        update.num_changed_entries = Some(1);
        update.string_data = Some(b"foo".to_vec());
        let mut up_payload = Vec::new();
        update.encode(&mut up_payload).unwrap();
        let mut packet2 = Vec::new();
        write_var(&mut packet2, msg::SvcMessages::SvcUpdateStringTable as u32);
        write_var(&mut packet2, up_payload.len() as u32);
        packet2.extend_from_slice(&up_payload);

        let updates2 = tables.parse_packet(&packet2);
        assert_eq!(1, updates2.len());
        let tbl = tables.get("test").unwrap();
        assert_eq!(tbl.entries.get(&0).unwrap().value, "foo");
    }
}
