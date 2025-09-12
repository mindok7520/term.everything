use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

type WlShmPoolId = u32;

/// Implements the wl_buffer object.
/// A wl_buffer represents a block of memory shared between the client and server.
/// This object is mostly a handle with metadata.
pub struct WlBuffer {
    // A reference back to the pool that created this buffer.
    parent_pool_id: WlShmPoolId,
    // Metadata about this specific buffer's view into the pool's memory.
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    // format: u32,
}

impl WlBuffer {
    pub fn new(
        parent_pool_id: WlShmPoolId,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        _format: u32,
    ) -> Self {
        Self {
            parent_pool_id,
            offset,
            width,
            height,
            stride,
            // format,
        }
    }

    fn handle_destroy(&self, client: &mut WaylandClient, self_id: u32) {
        println!("[wl_buffer] destroy: buffer {}", self_id);

        // This is the tricky part. We need to get a mutable reference to the parent pool
        // and call a method on it. This requires careful architecture in Rust to avoid
        // borrowing issues (e.g., using interior mutability like RefCell, or structuring
        // the main loop to handle this).

        // For this translation, we'll represent it conceptually.
        client.signal_buffer_destroyed(self.parent_pool_id, self_id);
    }
}

impl WaylandObject for WlBuffer {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const DESTROY: u16 = 0;

        match request.opcode {
            DESTROY => {
                self.handle_destroy(client, request.object_id);
                // After destroy, the object should be removed from the client's map.
                // The main loop would handle this.
            }
            _ => {
                eprintln!("[wl_buffer] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
