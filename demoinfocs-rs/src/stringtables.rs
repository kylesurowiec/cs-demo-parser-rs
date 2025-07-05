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
}

#[derive(Default)]
pub struct StringTables {
    tables: HashMap<i32, StringTable>,
}

impl StringTables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_create_string_table(&mut self, msg: &msg::CsvcMsgCreateStringTable) {
        let id = self.tables.len() as i32;
        let table = StringTable {
            name: msg.name.clone().unwrap_or_default(),
            ..Default::default()
        };
        self.tables.insert(id, table);
    }

    pub fn on_update_string_table(&mut self, msg: &msg::CsvcMsgUpdateStringTable) {
        if let Some(id) = msg.table_id {
            if let Some(table) = self.tables.get_mut(&id) {
                if let Some(data) = &msg.string_data {
                    let entry = StringTableEntry {
                        value: String::from_utf8_lossy(data).into_owned(),
                        user_data: data.clone(),
                    };
                    let idx = table.entries.len() as i32;
                    table.entries.insert(idx, entry);
                }
            }
        }
    }

    pub fn parse_svc_message(&mut self, typ: msg::SvcMessages, data: &[u8]) {
        match typ {
            | msg::SvcMessages::SvcCreateStringTable => {
                if let Ok(m) = msg::CsvcMsgCreateStringTable::decode(data) {
                    self.on_create_string_table(&m);
                }
            },
            | msg::SvcMessages::SvcUpdateStringTable => {
                if let Ok(m) = msg::CsvcMsgUpdateStringTable::decode(data) {
                    self.on_update_string_table(&m);
                }
            },
            | _ => {},
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

impl StringTables {
    pub fn parse_packet(&mut self, data: &[u8]) {
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
                self.parse_svc_message(t, msg_buf);
            }
        }
    }
}
