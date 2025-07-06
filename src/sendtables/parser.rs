use prost::Message as _;

use crate::bitreader::BitReader;
use crate::proto::msg::cs_demo_parser_rs as proto_msg;
use crate::proto::msg::cs_demo_parser_rs::CsvcMsgSendTable;

use super::entity::FlattenedPropEntry;
use super::propdecoder::{
    PROP_TYPE_ARRAY, PROP_TYPE_DATATABLE, SendPropertyFlags, SendTableProperty,
};
use super::serverclass::ServerClass;
use std::collections::HashMap;
use std::io::Read;

#[derive(Default)]
pub struct Parser {
    send_tables: Vec<SendTable>,
    server_classes: Vec<ServerClass>,
    instance_baselines: HashMap<i32, Vec<u8>>,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn server_classes(&self) -> &[ServerClass] {
        &self.server_classes
    }

    pub fn parse_packet(&mut self, data: &[u8]) -> Result<(), prost::DecodeError> {
        let mut r = BitReader::new_small(data);
        loop {
            let t = proto_msg::SvcMessages::try_from(r.read_varint32() as i32)
                .map_err(|_| prost::DecodeError::new("invalid message"))?;
            if t != proto_msg::SvcMessages::SvcSendTable {
                panic!("Expected SendTable message");
            }
            let size = r.read_varint32() as usize;
            let mut bytes = vec![0u8; size];
            for b in &mut bytes {
                *b = r.read_int(8) as u8;
            }
            let st = CsvcMsgSendTable::decode(&bytes[..])?;
            if st.is_end.unwrap_or(false) {
                break;
            }
            let mut table = SendTable {
                name: st.net_table_name.unwrap_or_default(),
                ..Default::default()
            };
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
            let mut sc = ServerClass {
                id,
                name,
                data_table_id: dt_id,
                data_table_name: dt_name,
                ..Default::default()
            };
            if let Some(b) = self.instance_baselines.remove(&id) {
                sc.instance_baseline = b;
            }
            self.server_classes.push(sc);
        }
        for i in 0..count {
            self.flatten_data_table(i);
        }
        Ok(())
    }

    pub fn set_instance_baseline(&mut self, sc_id: i32, data: Vec<u8>) {
        if let Some(sc) = self.server_classes.get_mut(sc_id as usize) {
            sc.instance_baseline = data;
        } else {
            self.instance_baselines.insert(sc_id, data);
        }
    }

    pub fn class_bits(&self) -> u32 {
        ((self.server_classes.len() as f32).log2().ceil()) as u32
    }

    pub fn read_enter_pvs<R: Read>(
        &mut self,
        reader: &mut BitReader<R>,
        entity_id: i32,
        existing: &mut HashMap<i32, super::entity::Entity>,
    ) -> super::entity::Entity {
        use crate::constants::ENTITY_HANDLE_SERIAL_NUMBER_BITS;
        let class_id = reader.read_int(self.class_bits()) as i32;
        let serial = reader.read_int(ENTITY_HANDLE_SERIAL_NUMBER_BITS) as i32;
        if let Some(ent) = existing.get_mut(&entity_id) {
            if ent.serial_num() == serial {
                ent.apply_update(reader);
                return ent.clone();
            }
            existing.remove(&entity_id);
        }
        let ent = self.server_classes[class_id as usize].new_entity(reader, entity_id, serial);
        existing.insert(entity_id, ent.clone());
        ent
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
