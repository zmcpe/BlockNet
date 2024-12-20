use binary::w32;
use derive::{Decode, Encode, Packet};
use crate::types::ItemStack;

/// Sent by the server to set the creative inventory's content for a player. Introduced in 1.16,
/// this packet replaces the previous method - sending an InventoryContent packet with creative
/// inventory window ID. As of v1.16.100, this packet must be sent during the login sequence. Not
/// sending it will stop the client from joining the server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CreativeContent<'a> {
    pub items: Vec<CreativeItem<'a>>,
}

/// A creative item present in the creative inventory.
#[derive(Debug, Clone, Encode, Decode)]
pub struct CreativeItem<'a> {
    /// A unique ID for the creative item. It has to be unique for each creative item sent to the
    /// client. An incrementing ID per creative item does the job.
    pub creative_item_network_id: w32,
    /// The item that should be added to the creative inventory.
    pub item: ItemStack<'a>,
}
