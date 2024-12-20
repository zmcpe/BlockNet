use binary::{v32, v64};
use derive::{Decode, Encode, Packet};

/// Sent by the server to damage the player's armour after being hit. The packet should never be
/// used by servers as it hands the responsibility over to the player completely, while the server
/// can easily reliably update the armour damage of players itself.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct HurtArmour {
    /// The cause of the damage dealt to the armour.
    pub cause: v32,
    /// The amount of damage points that was dealt to the player. The damage to the armour will be
    /// calculated by the client based upon this damage, and will also be based upon any
    /// enchantments that the armour may have.
    pub damage: v32,
    /// A bitset of all armour slots affected.
    pub armour_slots: v64,
}
