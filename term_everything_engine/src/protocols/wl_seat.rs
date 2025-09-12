use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};
use crate::protocols::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer};

// From the XML protocol file
mod capability {
    pub const POINTER: u32 = 1;
    pub const KEYBOARD: u32 = 2;
}

/// Implements the wl_seat Wayland global.
pub struct WlSeat;

impl WlSeat {
    pub fn new() -> Self {
        Self
    }

    pub fn on_bind(&self, client: &mut WaylandClient, self_id: u32) {
        println!("[wl_seat] Client bound, sending capabilities");
        let capabilities = capability::POINTER | capability::KEYBOARD;
        // conceptual event sending
        // client.send_event(self_id, WlSeat::Events::Capabilities { capabilities });
        // client.send_event(self_id, WlSeat::Events::Name { name: "seat0".to_string() });
    }

    fn handle_get_pointer(&self, client: &mut WaylandClient, new_pointer_id: u32) {
        println!("[wl_seat] get_pointer: creating pointer {}", new_pointer_id);
        let pointer = WlPointer::new();
        client.add_object(new_pointer_id, Box::new(pointer));
    }

    fn handle_get_keyboard(&self, client: &mut WaylandClient, new_keyboard_id: u32) {
        println!("[wl_seat] get_keyboard: creating keyboard {}", new_keyboard_id);
        let keyboard = WlKeyboard::new();
        // The keyboard needs to send the keymap as soon as it's created.
        keyboard.on_get_keyboard(client, new_keyboard_id);
        client.add_object(new_keyboard_id, Box::new(keyboard));
    }

    fn handle_get_touch(&self, _client: &mut WaylandClient, _self_id: u32) {
        println!("[wl_seat] get_touch: client requested unsupported touch capability");
        // client.post_error(self_id, wl_seat_error::MISSING_CAPABILITY, "no touch");
    }
}

impl WaylandObject for WlSeat {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const GET_POINTER: u16 = 0;
        const GET_KEYBOARD: u16 = 1;
        const GET_TOUCH: u16 = 2;
        const RELEASE: u16 = 3;

        match request.opcode {
            GET_POINTER => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                self.handle_get_pointer(client, new_id);
            }
            GET_KEYBOARD => {
                let new_id = request.args.get(0).cloned().unwrap_or(0);
                self.handle_get_keyboard(client, new_id);
            }
            GET_TOUCH => {
                self.handle_get_touch(client, request.object_id);
            }
            RELEASE => {
                 println!("[wl_seat] release");
            }
            _ => {
                eprintln!("[wl_seat] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
