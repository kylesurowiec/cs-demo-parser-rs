use std::collections::HashMap;
use std::fmt;

use super::entity::FlattenedPropEntry;

#[derive(Default, Clone)]
pub struct ServerClass {
    pub id: i32,
    pub name: String,
    pub data_table_id: i32,
    pub data_table_name: String,
    pub base_classes: Vec<ServerClass>,
    pub flattened_props: Vec<FlattenedPropEntry>,
    pub instance_baseline: Vec<u8>,
    pub preprocessed_baseline: HashMap<i32, super::entity::PropertyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyEntry {
    pub name: String,
    pub is_array: bool,
    pub prop_type: i32,
}

impl ServerClass {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn data_table_id(&self) -> i32 {
        self.data_table_id
    }
    pub fn data_table_name(&self) -> &str {
        &self.data_table_name
    }

    pub fn property_entries(&self) -> Vec<String> {
        self.flattened_props
            .iter()
            .map(|p| p.name.clone())
            .collect()
    }

    pub fn property_entry_definitions(&self) -> Vec<PropertyEntry> {
        self.flattened_props
            .iter()
            .map(|f| PropertyEntry {
                name: f.name.clone(),
                is_array: f.prop.raw_type == super::propdecoder::PROP_TYPE_ARRAY,
                prop_type: if f.prop.raw_type == super::propdecoder::PROP_TYPE_ARRAY {
                    f.array_element_prop.as_ref().map_or(0, |p| p.raw_type)
                } else {
                    f.prop.raw_type
                },
            })
            .collect()
    }
}

impl fmt::Display for ServerClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = if self.base_classes.is_empty() {
            "-".to_string()
        } else {
            self.base_classes
                .iter()
                .map(|b| b.name.clone())
                .collect::<Vec<_>>()
                .join("\n\t\t")
        };
        let props = if self.flattened_props.is_empty() {
            "-".to_string()
        } else {
            self.flattened_props
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join("\n\t\t")
        };
        write!(
            f,
            "serverClass: id={} name={}\n\tdataTableId={}\n\tdataTableName={}\n\tbaseClasses:\n\t\t{}\n\tproperties:\n\t\t{}",
            self.id,
            self.name,
            self.data_table_id,
            self.data_table_name,
            base,
            props
        )
    }
}
