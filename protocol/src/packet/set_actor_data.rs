use binary::w64;
use derive::{Decode, Encode, Packet};

use crate::types::entity_data::{EntityMetadata, EntityProperties};

/// Sent by the server to update the entity metadata of an entity. It includes flags such as if the
/// entity is on fire, but also properties such as the air it has left until it starts drowning.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetActorData<'a> {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// A map of entity metadata, which includes flags and data properties that alter in particular
    /// the way the player looks. Flags include ones such as 'on fire' and 'sprinting'. The meta
    /// values are indexed by their property key.
    pub entity_metadata: EntityMetadata<'a>,
    /// A list of properties that the entity inhibits. These properties define specific attributes
    /// of the entity.
    pub entity_properties: EntityProperties,
    /// The server tick at which the packet was sent. It is used in relation to
    /// CorrectPlayerMovePrediction.
    pub tick: w64,
}
