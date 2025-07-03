use crate::sendtables::entity::{Entity, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HostageState {
    Idle = 0,
    BeingUntied = 1,
    GettingPickedUp = 2,
    BeingCarried = 3,
    FollowingPlayer = 4,
    GettingDropped = 5,
    Rescued = 6,
    Dead = 7,
}

#[derive(Debug, Clone, Default)]
pub struct Hostage {
    pub entity: Option<Entity>,
}

impl Hostage {
    pub fn position(&self) -> Vector {
        self.entity
            .as_ref()
            .map(|e| e.position())
            .unwrap_or_default()
    }
}
