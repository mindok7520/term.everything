use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

// --- Placeholders ---
pub struct XdgToplevel;
impl WaylandObject for XdgToplevel {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}
pub struct XdgPopup;
impl WaylandObject for XdgPopup {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) { /* ... */ }
}
// ---

/// Implements the xdg_surface object.
/// This object is the bridge between a wl_surface and a desktop window role.
pub struct XdgSurface {
    // The ID of the wl_surface this xdg_surface is associated with.
    // This would be set upon creation.
    surface_id: u32,
}

impl XdgSurface {
    pub fn new(surface_id: u32) -> Self {
        Self { surface_id }
    }

    fn handle_get_toplevel(&self, client: &mut WaylandClient, new_toplevel_id: u32) {
        println!("[xdg_surface] get_toplevel: creating toplevel {} for surface {}", new_toplevel_id, self.surface_id);

        // This conceptually assigns the 'toplevel' role to the wl_surface.
        // A real implementation would need a robust way to manage roles.
        if client.get_surface_role(self.surface_id).is_some() {
             println!("[xdg_surface] Error: surface {} already has a role.", self.surface_id);
             // client.post_error(...)
             return;
        }
        client.set_surface_role(self.surface_id, new_toplevel_id, "xdg_toplevel");

        let toplevel = XdgToplevel {};
        client.add_object(new_toplevel_id, Box::new(toplevel));

        // Immediately send a configure event to the client.
        self.send_configure(client);
        // Also send a configure event to the new toplevel object.
        // client.send_event(new_toplevel_id, XdgToplevel::Events::Configure { ... });
    }

    fn handle_get_popup(&self, client: &mut WaylandClient, new_popup_id: u32) {
        println!("[xdg_surface] get_popup: creating popup {}", new_popup_id);
        // Similar logic to get_toplevel
        let popup = XdgPopup {};
        client.add_object(new_popup_id, Box::new(popup));
    }

    fn handle_ack_configure(&self, serial: u32) {
        println!("[xdg_surface] ack_configure: received ack for serial {}", serial);
        // Here, the compositor knows the client has processed the configuration.
        // It might resolve a promise or future.
    }

    fn send_configure(&self, client: &mut WaylandClient) {
        let serial = client.next_serial(); // conceptual method
        println!("[xdg_surface] sending configure event with serial {}", serial);
        // client.send_event(self.id, XdgSurface::Events::Configure { serial });
    }
}

impl WaylandObject for XdgSurface {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const DESTROY: u16 = 0;
        const GET_TOPLEVEL: u16 = 1;
        const GET_POPUP: u16 = 2;
        const SET_WINDOW_GEOMETRY: u16 = 3;
        const ACK_CONFIGURE: u16 = 4;

        match request.opcode {
            GET_TOPLEVEL => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                self.handle_get_toplevel(client, new_id);
            }
            GET_POPUP => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                self.handle_get_popup(client, new_id);
            }
            ACK_CONFIGURE => {
                let serial = request.args.get(0).cloned().unwrap_or(0);
                self.handle_ack_configure(serial);
            }
            DESTROY => {
                println!("[xdg_surface] destroy");
            }
            _ => {
                 eprintln!("[xdg_surface] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
