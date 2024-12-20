use binary::w32;
use derive::{Decode, Encode, Packet};

use crate::types::inventory::Window;

/// Sent by the server to the client. It used to be used to link hot bar slots of the player to
/// actual slots in the inventory, but as of 1.2, this was changed and hot bar slots are no longer a
/// free floating part of the inventory. Since 1.2, the packet has been re-purposed, but its new
/// functionality is not clear.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PlayerHotBar {
    /// Before 1.2, this was the hot bar slot that is being linked to the inventory slot.
    pub selected_hotbar_slot: w32,
    /// The window that the hot bar slot is in.
    pub window: Window,
    /// The exact purpose of this field is unknown.
    pub select_hotbar_slot: bool,
}
