use crate::sendtables::entity::{Entity, Vector};

#[derive(Default)]
pub struct Inferno {
    pub entity: Option<Entity>,
}

impl Inferno {
    pub fn position(&self) -> Vector {
        let _ = &self.entity;
        Vector::default()
    }
}
