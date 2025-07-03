use super::field::Field;

#[derive(Clone, Debug)]
pub struct Serializer {
    pub name: String,
    pub version: i32,
    pub fields: Vec<Field>,
}
