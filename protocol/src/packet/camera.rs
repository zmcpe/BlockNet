use binary::v64;
use derive::{Decode, Encode, Packet};

/// Sent by the server to use an Education Edition camera on a player. It produces an image
/// client-side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct Camera {
    /// The unique ID of the camera entity from which the picture was taken.
    pub camera_entity_unique_id: v64,
    /// The unique ID of the target player. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// player out for this field.
    pub target_player_unique_id: v64,
}
