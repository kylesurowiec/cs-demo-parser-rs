use super::field_type::FieldType;

#[derive(Clone, Debug)]
pub struct Field {
    pub var_name: String,
    pub var_type: String,
    pub field_type: FieldType,
    pub serializer_name: Option<String>,
    pub serializer_version: i32,
    pub bit_count: Option<i32>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub encode_flags: Option<i32>,
    pub var_encoder: Option<String>,
}
