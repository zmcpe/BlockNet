use crate::types::Vec3;
use derive::{Decode, Encode, Packet};

/// Sent by the server to spawn an outlined cube on client-side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ClientBoundDebugRenderer {
    /// The type of action to perform on the renderer, usually to add or clear a cube.
    pub render_type: ClientBoundDebugRendererType,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = i32)]
pub enum ClientBoundDebugRendererType {
    None,
    Clear,
    AddCube(AddCube),
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct AddCube {
    /// The text that is displayed above the debug.
    pub text: String,
    /// The position to spawn the debug on.
    pub position: Vec3,
    /// The red value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub red: f32,
    /// The green value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub green: f32,
    /// The blue value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub blue: f32,
    /// The alpha value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub alpha: f32,
    /// The duration the debug will last in the world for. It is measured in milliseconds.
    pub duration: i64,
}
