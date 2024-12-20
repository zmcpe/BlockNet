use binary::{v64, w32};
use derive::{Decode, Encode, Packet};
use crate::types::UBlockPos;
use crate::types::world::UpdateBlockTransition;

/// Sent by the server to synchronise the falling of a falling block entity with the transitioning
/// back and forth from and to a solid block. It is used to prevent the entity from flickering, and
/// is used in places such as the pushing of blocks with pistons.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct UpdateBlockSynced {
    /// The block position at which a block is updated.
    pub position: UBlockPos,
    /// The runtime ID of the new block that is placed at position.
    pub new_block_runtime_id: w32,
    /// A combination of `BlockUpdate` flags that specify the way the block is updated client-side.
    /// Typically, sending only the `Network` flag is sufficient.
    pub flags: w32,
    /// The world layer on which the block is updated. For most blocks, this is the first layer, as
    /// that layer is the default layer to place blocks on.
    pub layer: w32,
    /// The unique ID of the falling block entity that the block transitions to or that the entity
    /// transitions from. Note that for both possible values for TransitionType, the
    /// `entity_unique_id` should point to the falling block entity involved.
    pub entity_unique_id: v64,
    /// The type of the transition that happened. It is either `BlockToEntity`, when a block placed
    /// becomes a falling entity, or `EntityToBlock`, when a falling entity hits the ground and
    /// becomes a solid block again.
    pub transition_type: UpdateBlockTransition,
}
