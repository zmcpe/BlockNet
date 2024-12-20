use derive::{Decode, Encode, Packet};

/// Used to make the client connect to a websocket server. This websocket server has the ability to
/// execute commands on the behalf of the client and it can listen for certain events fired by the
/// client.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AutomationClientConnect {
    /// The URI to make the client connect to. It can be, for example, 'localhost:8000/ws' to
    /// connect to a websocket server on the localhost at port 8000.
    pub server_uri: String,
}
