use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

// --- Placeholders for objects created by the compositor ---
pub struct WlSurface;
impl WaylandObject for WlSurface {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}

pub struct WlRegion;
impl WaylandObject for WlRegion {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}
// --- End Placeholders ---


/// Implements the wl_compositor Wayland global.
/// Its main role is to act as a factory for WlSurface and WlRegion objects.
pub struct WlCompositor;

impl WlCompositor {
    pub fn new() -> Self {
        Self
    }

    fn handle_create_surface(&self, client: &mut WaylandClient, new_surface_id: u32) {
        println!("[wl_compositor] Handling create_surface, creating surface {}", new_surface_id);
        let surface = WlSurface {};
        client.add_object(new_surface_id, Box::new(surface));
    }

    fn handle_create_region(&self, client: &mut WaylandClient, new_region_id: u32) {
        println!("[wl_compositor] Handling create_region, creating region {}", new_region_id);
        let region = WlRegion {};
        client.add_object(new_region_id, Box::new(region));
    }
}

impl WaylandObject for WlCompositor {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        // Opcodes for wl_compositor requests from the protocol XML.
        const CREATE_SURFACE: u16 = 0;
        const CREATE_REGION: u16 = 1;

        match request.opcode {
            CREATE_SURFACE => {
                if let Some(new_id) = request.args.get(0) {
                    self.handle_create_surface(client, *new_id);
                }
            }
            CREATE_REGION => {
                if let Some(new_id) = request.args.get(0) {
                    self.handle_create_region(client, *new_id);
                }
            }
            _ => {
                eprintln!("[wl_compositor] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
