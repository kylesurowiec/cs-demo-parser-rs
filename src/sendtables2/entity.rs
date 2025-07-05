use super::class::Class;

#[derive(Clone, Debug)]
pub struct Entity {
    pub index: i32,
    pub serial: i32,
    pub class: Class,
}
