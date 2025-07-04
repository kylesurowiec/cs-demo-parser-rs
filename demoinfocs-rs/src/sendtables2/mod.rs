use crate::proto::msg::all as msg;
use prost::Message;
use std::collections::HashMap;

mod class;
mod entity;
mod field;
mod field_type;
mod huffman;
pub mod proto;
mod reader;
mod serializer;

pub use class::Class;
pub use entity::Entity;
pub use field::Field;
pub use field_type::FieldType;
pub use serializer::Serializer;

/// Minimal parser for Source2 send tables.
#[derive(Default)]
pub struct Parser {
    class_id_size: u32,
    serializers: HashMap<String, Serializer>,
    classes_by_id: HashMap<i32, Class>,
    classes_by_name: HashMap<String, Class>,
    class_baselines: HashMap<i32, Vec<u8>>,
    entities: HashMap<i32, Entity>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            class_id_size: 0,
            serializers: HashMap::new(),
            classes_by_id: HashMap::new(),
            classes_by_name: HashMap::new(),
            class_baselines: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    /// Handles CSVCMsg_ServerInfo and extracts the class id size.
    pub fn on_server_info(&mut self, msg: &msg::CsvcMsgServerInfo) {
        if let Some(max) = msg.max_classes {
            self.class_id_size = ((max as f32).log2().floor() as u32) + 1;
        }
    }

    /// Parses flattened serializer packets using a minimal subset of the
    /// Source2 send table format.
    pub fn parse_packet(&mut self, data: &[u8]) -> Result<(), prost::DecodeError> {
        // first bytes are a varint length prefix
        let mut slice = data;
        let len = read_var_uint32(&mut slice) as usize;
        if slice.len() < len {
            return Ok(()); // nothing to do
        }
        let (msg_buf, _rest) = slice.split_at(len);
        let msg = proto::CsvcMsgFlattenedSerializer::decode(msg_buf)?;

        let mut fields = Vec::new();
        for f in &msg.fields {
            let var_name = f
                .var_name_sym
                .and_then(|s| msg.symbols.get(s as usize).cloned())
                .unwrap_or_default();
            let var_type = f
                .var_type_sym
                .and_then(|s| msg.symbols.get(s as usize).cloned())
                .unwrap_or_default();
            let serializer_name = f
                .field_serializer_name_sym
                .and_then(|s| msg.symbols.get(s as usize).cloned());
            let var_encoder = f
                .var_encoder_sym
                .and_then(|s| msg.symbols.get(s as usize).cloned());
            let field = Field {
                var_name,
                var_type: var_type.clone(),
                field_type: FieldType::new(&var_type),
                serializer_name,
                serializer_version: f.field_serializer_version.unwrap_or_default(),
                bit_count: f.bit_count,
                low_value: f.low_value,
                high_value: f.high_value,
                encode_flags: f.encode_flags,
                var_encoder,
            };
            fields.push(field);
        }

        for s in msg.serializers {
            let name = s
                .serializer_name_sym
                .and_then(|sym| msg.symbols.get(sym as usize).cloned())
                .unwrap_or_default();
            let mut ser = Serializer {
                name: name.clone(),
                version: s.serializer_version.unwrap_or_default(),
                fields: Vec::new(),
            };
            for idx in s.fields_index {
                if let Some(f) = fields.get(idx as usize) {
                    ser.fields.push(f.clone());
                }
            }
            self.serializers.insert(name, ser);
        }

        Ok(())
    }

    pub fn class_id_size(&self) -> u32 {
        self.class_id_size
    }

    pub fn serializer(&self, name: &str) -> Option<&Serializer> {
        self.serializers.get(name)
    }

    pub fn entity(&self, index: i32) -> Option<&Entity> {
        self.entities.get(&index)
    }

    /// Handles CSVCMsg_ClassInfo and registers classes with their serializers.
    pub fn on_class_info(&mut self, msg: &msg::CsvcMsgClassInfo) {
        for c in &msg.classes {
            let class_id = c.class_id.unwrap_or_default();
            let name = c.class_name.clone().unwrap_or_default();
            let serializer = self.serializers.get(&name).cloned();
            let class = Class {
                class_id,
                name: name.clone(),
                serializer,
            };
            self.classes_by_id.insert(class_id, class.clone());
            self.classes_by_name.insert(name, class);
        }
    }

    /// Stores baseline data for a given class id.
    pub fn set_instance_baseline(&mut self, class_id: i32, data: Vec<u8>) {
        self.class_baselines.insert(class_id, data);
    }

    /// Parses a PacketEntities message and returns entity operations.
    pub fn parse_packet_entities(
        &mut self,
        msg: &msg::CsvcMsgPacketEntities,
    ) -> Vec<(Entity, crate::sendtables::EntityOp)> {
        use crate::sendtables::EntityOp;
        let mut events = Vec::new();
        if let Some(data) = msg.entity_data.as_ref() {
            let mut r = reader::Reader::new(data);
            let mut index: i32 = -1;
            let mut updates = msg.updated_entries.unwrap_or(0);
            while updates > 0 {
                let next = index + r.read_ubit_var() as i32 + 1;
                index = next;
                let cmd = r.read_bits(2);
                if cmd & 0x01 == 0 {
                    if cmd & 0x02 != 0 {
                        let class_id = r.read_bits(self.class_id_size) as i32;
                        let serial = r.read_bits(17) as i32;
                        let _ = r.read_var_uint32();
                        if let Some(class) = self.classes_by_id.get(&class_id) {
                            let ent = Entity {
                                index,
                                serial,
                                class: class.clone(),
                            };
                            self.entities.insert(index, ent.clone());
                            events.push((ent, EntityOp::CREATED | EntityOp::ENTERED));
                        }
                    } else {
                        if let Some(ent) = self.entities.get(&index).cloned() {
                            events.push((ent, EntityOp::UPDATED));
                        }
                    }
                } else if cmd & 0x02 != 0 {
                    if let Some(ent) = self.entities.remove(&index) {
                        events.push((ent, EntityOp::DELETED | EntityOp::LEFT));
                    }
                }
                updates -= 1;
            }
        }
        events
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
