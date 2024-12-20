use binary::w64;
use derive::{Decode, Encode, Packet};

/// Sent by the client in response to a PlayStatus packet with the status set to PlayerSpawn. The
/// packet marks the moment at which the client is fully initialised and can receive any packet
/// without discarding it.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetLocalPlayerAsInitialised {
    /// The entity runtime ID the player was assigned earlier in the login sequence in the StartGame
    /// packet.
    pub entity_runtime_id: w64,
}
