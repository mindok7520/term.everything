use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

/// Implements the xdg_toplevel role object.
/// This holds the state for a standard desktop window.
#[derive(Default)]
pub struct XdgToplevel {
    title: Option<String>,
    app_id: Option<String>,
    min_size: (i32, i32),
    max_size: (i32, i32),
    maximized: bool,
    fullscreen: bool,
}

impl XdgToplevel {
    pub fn new() -> Self {
        Default::default()
    }

    fn handle_set_title(&mut self, title: String) {
        println!("[xdg_toplevel] set_title: {}", title);
        self.title = Some(title);
    }

    fn handle_set_app_id(&mut self, app_id: String) {
        println!("[xdg_toplevel] set_app_id: {}", app_id);
        self.app_id = Some(app_id);
    }

    fn handle_set_maximized(&mut self, client: &mut WaylandClient) {
        println!("[xdg_toplevel] set_maximized");
        self.maximized = true;
        // In a real implementation, this would trigger a re-configure of the surface.
        // client.send_event(self.id, XdgToplevel::Events::Configure { ... });
    }

    fn handle_unset_maximized(&mut self, client: &mut WaylandClient) {
        println!("[xdg_toplevel] unset_maximized");
        self.maximized = false;
        // ... trigger re-configure
    }
}

impl WaylandObject for XdgToplevel {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const DESTROY: u16 = 0;
        const SET_PARENT: u16 = 1;
        const SET_TITLE: u16 = 2;
        const SET_APP_ID: u16 = 3;
        const SHOW_WINDOW_MENU: u16 = 4;
        const MOVE: u16 = 5;
        const RESIZE: u16 = 6;
        const SET_MAX_SIZE: u16 = 7;
        const SET_MIN_SIZE: u16 = 8;
        const SET_MAXIMIZED: u16 = 9;
        const UNSET_MAXIMIZED: u16 = 10;
        const SET_FULLSCREEN: u16 = 11;
        const UNSET_FULLSCREEN: u16 = 12;
        const SET_MINIMIZED: u16 = 13;

        match request.opcode {
            SET_TITLE => {
                // This is a simplification. The arg is a string, which isn't
                // handled by the basic `Request` struct yet.
                self.handle_set_title("...".to_string());
            }
            SET_APP_ID => {
                self.handle_set_app_id("...".to_string());
            }
            SET_MAXIMIZED => {
                self.handle_set_maximized(client);
            }
            UNSET_MAXIMIZED => {
                self.handle_unset_maximized(client);
            }
            DESTROY => {
                println!("[xdg_toplevel] destroy");
            }
            _ => {
                println!("[xdg_toplevel] Received unhandled opcode: {}", request.opcode);
            }
        }
    }
}
