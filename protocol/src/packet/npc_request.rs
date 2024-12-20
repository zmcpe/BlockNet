use binary::w64;
use derive::{Decode, Encode, Packet};

/// Sent by the client when it interacts with an NPC. The packet is specifically made for Education
/// Edition, where NPCs are available to use.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct NPCRequest {
    /// The runtime ID of the NPC entity that the player interacted with. It is the same as sent by
    /// the server when spawning the entity.
    pub entity_runtime_id: w64,
    /// The type of the request, which depends on the permission that the player has. It will be
    /// either a type that indicates that the NPC should show its dialog, or that it should open the
    /// editing window.
    pub request_type: NPCRequestAction,
    /// The command string set in the NPC. It may consist of multiple commands, depending on what
    /// the player set in it.
    pub command_string: String,
    /// The type of the action to execute.
    pub action_type: u8,
    /// The name of the scene. This can be left empty to specify the last scene that the player was
    /// sent.
    pub scene_name: String,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum NPCRequestAction {
    SetActions,
    ExecuteAction,
    ExecuteClosingCommands,
    SetName,
    SetSkin,
    SetInteractText,
    ExecuteOpeningCommands,
}
