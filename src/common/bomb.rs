use super::Player;
use crate::sendtables::entity::Vector;
use crate::sendtables2::Entity;

#[derive(Default)]
pub struct Bomb {
    pub entity: Option<Entity>,
    pub last_on_ground_position: Vector,
    pub carrier: Option<Player>,
}

impl Bomb {
    pub fn position(&self) -> Vector {
        if let Some(carrier) = &self.carrier {
            carrier.position()
        } else {
            self.last_on_ground_position.clone()
        }
    }
}
