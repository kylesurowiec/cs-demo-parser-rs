use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EntityOp: u8 {
        const NONE = 0x00;
        const CREATED = 0x01;
        const UPDATED = 0x02;
        const DELETED = 0x04;
        const ENTERED = 0x08;
        const LEFT = 0x10;
    }
}
