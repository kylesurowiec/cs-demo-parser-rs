use super::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct Class {
    pub class_id: i32,
    pub name: String,
    pub serializer: Option<Serializer>,
}
