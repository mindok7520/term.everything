use std::collections::HashMap;
use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

type SurfaceId = u32;
type ClientId = u32; // Using the client's FD as a unique ID.

/// Implements the wl_pointer object.
/// This object represents a mouse/pointer device. It handles setting the
/// cursor image and is responsible for sending pointer events.
#[derive(Default)]
pub struct WlPointer {
    // Tracks the cursor surface for each client.
    // The key is a unique client identifier.
    cursor_surface_per_client: HashMap<ClientId, SurfaceId>,
    // The global position of the pointer on the virtual screen.
    x: f64,
    y: f64,
}

impl WlPointer {
    pub fn new() -> Self {
        Default::default()
    }

    fn handle_set_cursor(
        &mut self,
        client: &mut WaylandClient,
        _serial: u32,
        surface_id: SurfaceId,
        hotspot_x: i32,
        hotspot_y: i32
    ) {
        println!(
            "[wl_pointer] set_cursor: surface={}, hotspot=({},{})",
            surface_id, hotspot_x, hotspot_y
        );

        // Update the cursor surface for this client.
        self.cursor_surface_per_client.insert(client.get_id(), surface_id);

        // Assign the 'cursor' role to the surface.
        // This is a conceptual call.
        client.set_surface_role(surface_id, 0, "cursor");
    }
}

impl WaylandObject for WlPointer {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const SET_CURSOR: u16 = 0;
        const RELEASE: u16 = 1;

        match request.opcode {
            SET_CURSOR => {
                let serial = request.args.get(0).cloned().unwrap_or(0);
                let surface_id = request.args.get(1).cloned().unwrap_or(0);
                let hotspot_x = request.args.get(2).cloned().unwrap_or(0) as i32;
                let hotspot_y = request.args.get(3).cloned().unwrap_or(0) as i32;
                self.handle_set_cursor(client, serial, surface_id, hotspot_x, hotspot_y);
            }
            RELEASE => {
                println!("[wl_pointer] release");
            }
            _ => {
                eprintln!("[wl_pointer] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
