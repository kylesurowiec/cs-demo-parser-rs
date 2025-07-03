use crate::bitreader::BitReader;
use crate::proto::msg::{self, all as proto};
use prost::Message;

use super::entity::FlattenedPropEntry;
use super::propdecoder::{SendPropertyFlags, SendTableProperty, PROP_TYPE_ARRAY, PROP_TYPE_DATATABLE};
use super::serverclass::ServerClass;

#[derive(Default)]
pub struct Parser {
    send_tables: Vec<SendTable>,
    server_classes: Vec<ServerClass>,
}

#[derive(Default)]
struct SendTable {
    name: String,
    properties: Vec<SendPropEntry>,
}

#[derive(Clone)]
struct SendPropEntry {
    prop: SendTableProperty,
    name: String,
    data_table_name: String,
}

impl Default for SendPropEntry {
    fn default() -> Self {
        Self {
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: 0,
            },
            name: String::new(),
            data_table_name: String::new(),
        }
    }
}

impl Parser {
    pub fn new() -> Self { Self::default() }

    pub fn server_classes(&self) -> &[ServerClass] { &self.server_classes }

    pub fn parse_packet(&mut self, data: &[u8]) -> Result<(), prost::DecodeError> {
        let mut r = BitReader::new_small(data);
        loop {
            let t = msg::SvcMessages::from_i32(r.read_varint32() as i32)
                .ok_or_else(|| prost::DecodeError::new("invalid message"))?;
            if t != msg::SvcMessages::SvcSendTable {
                panic!("Expected SendTable message");
            }
            let size = r.read_varint32() as usize;
            let mut bytes = vec![0u8; size];
            for b in &mut bytes {
                *b = r.read_int(8) as u8;
            }
            let st = proto::CsvcMsgSendTable::decode(&bytes[..])?;
            if st.is_end.unwrap_or(false) {
                break;
            }
            let mut table = SendTable { name: st.net_table_name.unwrap_or_default(), ..Default::default() };
            for p in st.props {
                let prop = SendTableProperty {
                    flags: SendPropertyFlags::from_bits_truncate(p.flags.unwrap_or(0) as u32),
                    low_value: p.low_value.unwrap_or(0.0),
                    high_value: p.high_value.unwrap_or(0.0),
                    number_of_bits: p.num_bits.unwrap_or(0) as u32,
                    number_of_elements: p.num_elements.unwrap_or(0),
                    priority: p.priority.unwrap_or(0),
                    raw_type: p.r#type.unwrap_or(0),
                };
                table.properties.push(SendPropEntry {
                    prop,
                    name: p.var_name.unwrap_or_default(),
                    data_table_name: p.dt_name.unwrap_or_default(),
                });
            }
            self.send_tables.push(table);
        }
        let count = r.read_int(16) as usize;
        for _ in 0..count {
            let id = r.read_int(16) as i32;
            let name = r.read_string();
            let dt_name = r.read_string();
            let dt_id = self
                .send_tables
                .iter()
                .position(|t| t.name == dt_name)
                .unwrap_or(0) as i32;
            self.server_classes.push(ServerClass {
                id,
                name,
                data_table_id: dt_id,
                data_table_name: dt_name,
                ..Default::default()
            });
        }
        for i in 0..count {
            self.flatten_data_table(i);
        }
        Ok(())
    }

    fn flatten_data_table(&mut self, sc_idx: usize) {
        let table_id = self.server_classes[sc_idx].data_table_id as usize;
        let mut props = Vec::new();
        self.gather_props(table_id, "", &mut props);
        self.server_classes[sc_idx].flattened_props = props;
    }

    fn gather_props(&self, tab_idx: usize, prefix: &str, out: &mut Vec<FlattenedPropEntry>) {
        let tab = &self.send_tables[tab_idx];
        for (i, p) in tab.properties.iter().enumerate() {
            if p.prop.flags.contains(SendPropertyFlags::INSIDEARRAY)
                || p.prop.flags.contains(SendPropertyFlags::EXCLUDE)
            {
                continue;
            }
            if p.prop.raw_type == PROP_TYPE_DATATABLE {
                if let Some(id) = self
                    .send_tables
                    .iter()
                    .position(|t| t.name == p.data_table_name)
                {
                    let mut new_prefix = prefix.to_string();
                    if !p.name.is_empty() {
                        new_prefix.push_str(&p.name);
                        new_prefix.push('.');
                    }
                    self.gather_props(id, &new_prefix, out);
                }
            } else {
                let array_prop = if p.prop.raw_type == PROP_TYPE_ARRAY && i > 0 {
                    Some(tab.properties[i - 1].prop.clone())
                } else {
                    None
                };
                out.push(FlattenedPropEntry {
                    name: format!("{}{}", prefix, p.name),
                    prop: p.prop.clone(),
                    array_element_prop: array_prop,
                });
            }
        }
    }
}
