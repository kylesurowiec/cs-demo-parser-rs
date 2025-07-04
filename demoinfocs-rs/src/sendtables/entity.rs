use super::propdecoder::{PropertyType, SendTableProperty};
use crate::bitreader::BitReader;
use crate::sendtables::propdecoder::PropertyDecoder;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PropertyValue {
    pub vector_val: Vector,
    pub int_val: i32,
    pub int64_val: i64,
    pub array_val: Vec<PropertyValue>,
    pub string_val: String,
    pub float_val: f32,
}

impl PropertyValue {
    pub fn bool_val(&self) -> bool {
        self.int_val > 0
    }
}

#[derive(Debug, Clone)]
pub struct FlattenedPropEntry {
    pub prop: SendTableProperty,
    pub array_element_prop: Option<SendTableProperty>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub entry: FlattenedPropEntry,
    pub value: PropertyValue,
}

impl Property {
    pub fn name(&self) -> &str {
        &self.entry.name
    }
    pub fn value(&self) -> &PropertyValue {
        &self.value
    }
    pub fn r#type(&self) -> PropertyType {
        self.entry.prop.raw_type.into()
    }
    pub fn array_element_type(&self) -> PropertyType {
        self.entry
            .array_element_prop
            .as_ref()
            .map(|p| p.raw_type.into())
            .unwrap_or(PropertyType::Int)
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub server_class: ServerClassRef,
    pub id: i32,
    pub serial_num: i32,
    pub props: Vec<Property>,
}

pub type ServerClassRef = Arc<super::serverclass::ServerClass>;

impl Entity {
    pub fn server_class(&self) -> ServerClassRef {
        self.server_class.clone()
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn serial_num(&self) -> i32 {
        self.serial_num
    }
    pub fn properties(&self) -> Vec<&Property> {
        self.props.iter().collect()
    }
    fn property(&self, name: &str) -> Option<&Property> {
        if self.server_class.flattened_props.is_empty() {
            // Tests may construct `Entity` instances manually without a fully
            // populated `ServerClass`. In that case fall back to a linear
            // search through all properties by name.
            return self.props.iter().find(|p| p.name() == name);
        }

        self.server_class
            .flattened_props
            .iter()
            .position(|p| p.name == name)
            .map(|idx| &self.props[idx])
    }
    pub fn property_public(&self, name: &str) -> Option<&Property> {
        self.property(name)
    }
    pub fn property_value(&self, name: &str) -> Option<PropertyValue> {
        self.property(name).map(|p| p.value.clone())
    }
    pub fn property_value_must(&self, name: &str) -> PropertyValue {
        self.property(name)
            .expect("property not found")
            .value
            .clone()
    }

    pub fn position(&self) -> Vector {
        self.property_value("m_vecOrigin")
            .map(|v| v.vector_val)
            .unwrap_or_default()
    }

    pub fn apply_update<R: Read>(&mut self, reader: &mut BitReader<R>) {
        let mut idx: i32 = -1;
        let new_way = reader.read_bit();
        let mut updated = Vec::new();

        loop {
            idx = read_field_index(reader, idx, new_way);
            if idx == -1 {
                break;
            }
            updated.push(idx as usize);
        }

        let decoder = PropertyDecoder;
        for i in updated {
            decoder.decode_prop(&mut self.props[i], reader);
        }
    }

    pub(super) fn initialize_baseline<R: Read>(
        &mut self,
        reader: &mut BitReader<R>,
    ) -> HashMap<i32, PropertyValue> {
        self.apply_update(reader);
        let mut map = HashMap::new();
        for (i, p) in self.props.iter().enumerate() {
            map.insert(i as i32, p.value.clone());
        }
        map
    }

    pub(super) fn apply_baseline(&mut self, baseline: &HashMap<i32, PropertyValue>) {
        for (idx, val) in baseline {
            if let Some(p) = self.props.get_mut(*idx as usize) {
                p.value = val.clone();
            }
        }
    }
}

fn read_field_index<R: Read>(reader: &mut BitReader<R>, last_index: i32, new_way: bool) -> i32 {
    if new_way && reader.read_bit() {
        return last_index + 1;
    }

    let mut res: u32;
    if new_way && reader.read_bit() {
        res = reader.read_int(3);
    } else {
        res = reader.read_int(7);
        match res & (32 | 64) {
            | 32 => res = (res & !96) | (reader.read_int(2) << 5),
            | 64 => res = (res & !96) | (reader.read_int(4) << 5),
            | 96 => res = (res & !96) | (reader.read_int(7) << 5),
            | _ => {},
        }
    }

    const FIELD_INDEX_END_MARKER: u32 = 0xfff;
    if res == FIELD_INDEX_END_MARKER {
        -1
    } else {
        last_index + 1 + res as i32
    }
}
