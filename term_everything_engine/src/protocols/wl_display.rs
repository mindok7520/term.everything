use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

// --- Placeholders for other objects this module interacts with ---
// We need a placeholder for WlRegistry to add it to the object map.
pub struct WlRegistry;
impl WaylandObject for WlRegistry {
    fn on_request(&mut self, _client: &mut WaylandClient, _request: Request) {
        // This is where wl_registry.bind would be handled.
        // It would look up the global in the client and add a new object reference.
    }
}
// --- End Placeholders ---


pub struct WlDisplay;

impl WlDisplay {
    pub fn new() -> Self {
        Self
    }

    fn handle_sync(&self, _client: &mut WaylandClient, callback_id: u32) {
        println!("[wl_display] Handling sync request, sending done to callback {}", callback_id);
        // client.send_event(callback_id, WlCallback::Events::Done { serial: 0 });
    }

    fn handle_get_registry(&self, client: &mut WaylandClient, registry_id: u32) {
        println!("[wl_display] Handling get_registry request, creating registry {}", registry_id);
        let registry = WlRegistry {};
        client.add_object(registry_id, Box::new(registry));

        // Get the list of globals from the client itself, which holds the canonical list.
        let globals = client.list_globals();

        for (global_id, interface_name, version) in globals {
             println!("[wl_display] Advertising global: {} (id {})", interface_name, global_id);
             // This would queue a 'global' event to be sent on the new registry object.
             // client.send_event(registry_id, WlRegistry::Events::Global {
             //     name: global_id,
             //     interface: interface_name.to_string(),
             //     version,
             // });
        }
    }
}

impl WaylandObject for WlDisplay {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const SYNC: u16 = 0;
        const GET_REGISTRY: u16 = 1;

        match request.opcode {
            SYNC => {
                if let Some(callback_id) = request.args.get(0) {
                    self.handle_sync(client, *callback_id);
                }
            }
            GET_REGISTRY => {
                if let Some(registry_id) = request.args.get(0) {
                    self.handle_get_registry(client, *registry_id);
                }
            }
            _ => {
                eprintln!("[wl_display] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
