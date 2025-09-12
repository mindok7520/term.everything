use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

// --- Placeholders ---
pub struct XdgSurface;
impl WaylandObject for XdgSurface {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}

pub struct XdgPositioner;
impl WaylandObject for XdgPositioner {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}
// ---

/// Implements the xdg_wm_base Wayland global.
/// This is the entry point for the xdg-shell window management protocol.
pub struct XdgWmBase;

impl XdgWmBase {
    pub fn new() -> Self {
        Self
    }

    fn handle_get_xdg_surface(
        &self,
        client: &mut WaylandClient,
        self_id: u32,
        new_xdg_surface_id: u32,
        surface_id: u32,
    ) {
        println!("[xdg_wm_base] get_xdg_surface: creating xdg_surface {} for wl_surface {}", new_xdg_surface_id, surface_id);

        // This is a conceptual implementation of the role check.
        // It requires a way to check and set a role on a wl_surface.
        if client.get_surface_role(surface_id).is_some() {
            println!("[xdg_wm_base] Error: wl_surface {} already has a role.", surface_id);
            // client.post_error(self_id, xdg_wm_base_error::ROLE, "surface already has a role");
            return;
        }

        // Associate the surface with the new xdg_surface role.
        client.set_surface_role(surface_id, new_xdg_surface_id, "xdg_surface");

        let xdg_surface = XdgSurface {};
        client.add_object(new_xdg_surface_id, Box::new(xdg_surface));
    }

    fn handle_create_positioner(&self, client: &mut WaylandClient, new_positioner_id: u32) {
        println!("[xdg_wm_base] create_positioner: creating positioner {}", new_positioner_id);
        let positioner = XdgPositioner {};
        client.add_object(new_positioner_id, Box::new(positioner));
    }

    fn handle_pong(&self, _serial: u32) {
        // A client sends this in response to a ping event.
        println!("[xdg_wm_base] pong received");
    }
}

impl WaylandObject for XdgWmBase {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const DESTROY: u16 = 0;
        const CREATE_POSITIONER: u16 = 1;
        const GET_XDG_SURFACE: u16 = 2;
        const PONG: u16 = 3;

        match request.opcode {
            GET_XDG_SURFACE => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                let surface_id = request.args.get(1).cloned().unwrap_or(0);
                self.handle_get_xdg_surface(client, request.object_id, new_id, surface_id);
            }
            CREATE_POSITIONER => {
                 let new_id = request.args.get(0).cloned().unwrap_or(0);
                 self.handle_create_positioner(client, new_id);
            }
            PONG => {
                let serial = request.args.get(0).cloned().unwrap_or(0);
                self.handle_pong(serial);
            }
            DESTROY => {
                println!("[xdg_wm_base] destroy");
            }
            _ => {
                eprintln!("[xdg_wm_base] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
