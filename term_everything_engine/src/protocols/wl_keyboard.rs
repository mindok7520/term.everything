use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

// From the XML protocol file
mod keymap_format {
    pub const NO_KEYMAP: u32 = 0;
    pub const XKB_V1: u32 = 1;
}

/// Implements the wl_keyboard object.
/// This object represents a keyboard device and is responsible for
/// sending the keymap and key events to the client.
pub struct WlKeyboard {
    // In a real implementation, the constructor would create a temporary
    // file with the XKB keymap and store the file descriptor and size here.
    keymap_fd: i32,
    keymap_size: u32,
}

impl WlKeyboard {
    pub fn new() -> Self {
        println!("[wl_keyboard] new: preparing keymap (stub)");
        // Conceptual representation of creating a temp file with the keymap.
        // These values would come from file system operations.
        Self {
            keymap_fd: -1, // Invalid FD placeholder
            keymap_size: 2429, // Size from the original project's resource
        }
    }

    /// This logic is called by wl_seat after a client gets a keyboard.
    pub fn on_get_keyboard(&self, client: &mut WaylandClient, self_id: u32) {
        println!("[wl_keyboard] on_get_keyboard: sending keymap to client for keyboard {}", self_id);

        // This would send the keymap event to the client.
        // client.send_event(self_id, WlKeyboard::Events::Keymap {
        //     format: keymap_format::XKB_V1,
        //     fd: self.keymap_fd,
        //     size: self.keymap_size,
        // });
    }
}

impl WaylandObject for WlKeyboard {
    fn on_request(&mut self, _client: &mut WaylandClient, request: Request) {
        const RELEASE: u16 = 0;

        match request.opcode {
            RELEASE => {
                println!("[wl_keyboard] release");
                // The object will be destroyed by the client's main loop.
            }
            _ => {
                eprintln!("[wl_keyboard] Received unknown opcode: {}", request.opcode);
            }
        }
    }
}
