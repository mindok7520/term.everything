use std::collections::HashMap;
use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};
use crate::protocols::wl_buffer::WlBuffer; // Import the concrete WlBuffer

type WlBufferId = u32;

#[derive(Debug, PartialEq)]
enum MapState {
    Mmapped,
    Destroyed,
    DestroyWhenBuffersEmpty,
}

struct BufferInfo {
    // This could hold more info if needed, but for now it's a marker.
}

/// Implements the wl_shm_pool object.
pub struct WlShmPool {
    pool_id: u32,
    client_state_ref: u32,
    map_state: MapState,
    buffers: HashMap<WlBufferId, BufferInfo>,
}

impl WlShmPool {
    pub fn new(client: &mut WaylandClient, pool_id: u32, fd: i32, size: i32) -> Self {
        println!("[wl_shm_pool] new: mmapping fd {} for pool {}", fd, pool_id);
        let success = true; // Assume success
        let map_state = if success { MapState::Mmapped } else { MapState::Destroyed };
        Self {
            pool_id,
            client_state_ref: client.get_client_state_ref(),
            map_state,
            buffers: HashMap::new(),
        }
    }

    fn handle_create_buffer(
        &mut self,
        client: &mut WaylandClient,
        new_buffer_id: u32,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32
    ) {
        println!("[wl_shm_pool] create_buffer: creating buffer {}", new_buffer_id);
        let buffer = WlBuffer::new(self.pool_id, offset, width, height, stride, format);
        client.add_object(new_buffer_id, Box::new(buffer));
        self.buffers.insert(new_buffer_id, BufferInfo{});
    }

    fn handle_resize(&mut self, new_size: i32) {
         if self.map_state != MapState::Destroyed {
            println!("[wl_shm_pool] resize: resizing pool {} to {} bytes", self.pool_id, new_size);
         }
    }

    fn handle_destroy(&mut self, client: &mut WaylandClient) {
        if self.buffers.is_empty() {
            self.unmap(client);
        } else {
            println!("[wl_shm_pool] destroy: has active buffers, will destroy when empty.");
            self.map_state = MapState::DestroyWhenBuffersEmpty;
        }
    }

    pub fn buffer_destroyed(&mut self, client: &mut WaylandClient, buffer_id: WlBufferId) {
        self.buffers.remove(&buffer_id);
        if self.map_state == MapState::DestroyWhenBuffersEmpty && self.buffers.is_empty() {
             self.unmap(client);
        }
    }

    fn unmap(&mut self, _client: &mut WaylandClient) {
        if self.map_state != MapState::Destroyed {
            println!("[wl_shm_pool] unmap: unmapping and destroying pool {}", self.pool_id);
            self.map_state = MapState::Destroyed;
        }
    }
}

impl WaylandObject for WlShmPool {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const CREATE_BUFFER: u16 = 0;
        const DESTROY: u16 = 1;
        const RESIZE: u16 = 2;

        match request.opcode {
            CREATE_BUFFER => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                let offset = request.args.get(1).cloned().unwrap_or(0) as i32;
                let width = request.args.get(2).cloned().unwrap_or(0) as i32;
                let height = request.args.get(3).cloned().unwrap_or(0) as i32;
                let stride = request.args.get(4).cloned().unwrap_or(0) as i32;
                let format = request.args.get(5).cloned().unwrap_or(0);
                self.handle_create_buffer(client, new_id, offset, width, height, stride, format);
            }
            RESIZE => {
                let new_size = request.args.get(0).cloned().unwrap_or(0) as i32;
                self.handle_resize(new_size);
            }
            DESTROY => {
                self.handle_destroy(client);
            }
            _ => {
                eprintln!("[wl_shm_pool] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}

impl Drop for WlShmPool {
    fn drop(&mut self) {
        if self.map_state != MapState::Destroyed {
             println!("[wl_shm_pool] Drop: unmapping pool {}", self.pool_id);
        }
    }
}
