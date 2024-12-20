use derive::{Decode, Encode, Packet};

#[derive(Debug, Clone, PartialEq, Encode, Decode, Packet)]
pub struct PlayerToggleCrafterSlotRequest {
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub slot: u8,
    pub disabled: bool,
}
