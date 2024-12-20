use binary::b64;
use derive::{Decode, Encode, Packet};
use crate::types::Magic;

#[derive(Debug, Encode, Decode, Packet)]
pub struct UnconnectedPing {
    pub ping_time: b64,
    pub magic: Magic,
    pub client_guid: b64
}