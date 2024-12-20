use derive::{Decode, Encode, Packet};

/// Allows the client to display dialog boxes for interacting with NPCs.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct NPCDialogue {
    /// The unique ID of the NPC being requested.
    pub entity_unique_id: u64,
    /// The type of action for the packet.
    pub action_type: NPCDialogueAction,
    /// The dialogue text that the client should see.
    pub dialogue: String,
    /// The identifier of the scene. If this is left empty, the client will use the last scene sent
    /// to it. (<https://docs.microsoft.com/en-us/minecraft/creator/documents/npcdialogue>)
    pub scene_name: String,
    /// The name of the NPC to be displayed to the client.
    pub npc_name: String,
    /// The JSON string of the buttons/actions the server can perform.
    pub action_json: String,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum NPCDialogueAction {
    Open,
    Close,
}
