use crate::wayland_client::WaylandClient;

// Represents a parsed message from the client
pub struct Request {
    pub object_id: u32,
    pub opcode: u16,
    pub args: Vec<u32>, // A simplification of argument types
}

/// A trait for any Wayland object that can handle requests.
/// This is the Rust equivalent of the Wayland_Object base class.
pub trait WaylandObject {
    /// Handles an incoming request from a client for this object.
    fn on_request(&mut self, client: &mut WaylandClient, request: Request);
}
