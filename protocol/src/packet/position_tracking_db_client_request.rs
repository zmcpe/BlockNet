use binary::v32;
use derive::{Decode, Encode, Packet};

/// Sent by the client to request the position and dimension of a 'tracking ID'. These IDs are
/// tracked in a database by the server. In 1.16, this is used for lodestones. The client will send
/// this request to find the position a lodestone compass needs to point to. If found, it will point
/// to the lodestone. If not, it will start spinning around. A PositionTrackingDBServerBroadcast
/// packet should be sent in response to this packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PositionTrackingDBClientRequest {
    /// The action that should be performed upon the receiving of the packet.
    pub request_action: PositionTrackingDBRequestAction,
    /// A unique ID used to identify the request. The server responds with a
    /// PositionTrackingDBServerBroadcast packet holding the same ID, so that the client can find
    /// out what that packet was in response to.
    pub tracking_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum PositionTrackingDBRequestAction {
    Query,
}
