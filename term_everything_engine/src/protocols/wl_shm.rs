use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};
use crate::protocols::wl_shm_pool::WlShmPool; // Import the concrete WlShmPool

/// Implements the wl_shm Wayland global.
pub struct WlShm;

impl WlShm {
    pub fn new() -> Self {
        Self
    }

    pub fn on_bind(&self, _client: &mut WaylandClient, _object_id: u32) {
        println!("[wl_shm] Client bound, advertising supported format ARGB8888");
        // client.send_event(object_id, WlShm::Events::Format { format: 1 });
    }

    fn handle_create_pool(&self, client: &mut WaylandClient, new_pool_id: u32, fd: i32, size: i32) {
        println!("[wl_shm] Handling create_pool, creating pool {} with fd {} and size {}", new_pool_id, fd, size);
        let pool = WlShmPool::new(client, new_pool_id, fd, size);
        client.add_object(new_pool_id, Box::new(pool));
    }
}

impl WaylandObject for WlShm {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const CREATE_POOL: u16 = 0;

        match request.opcode {
            CREATE_POOL => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                let size = request.args.get(1).cloned().unwrap_or(0) as i32;
                let fd = 0; // Placeholder for the claimed file descriptor.

                self.handle_create_pool(client, new_id, fd, size);
            }
            _ => {
                eprintln!("[wl_shm] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
