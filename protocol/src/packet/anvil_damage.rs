use derive::{Decode, Encode, Packet};
use crate::types::UBlockPos;

/// Sent by the client to request the dealing damage to an anvil. This packet is completely
/// pointless and the server should never listen to it.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AnvilDamage {
    /// The damage that the client requests to be dealt to the anvil.
    pub damage: u8,
    /// The position in the world that the anvil can be found at.
    pub anvil_position: UBlockPos,
}
