use crate::wayland_client::WaylandClient;
use crate::wayland_object::{Request, WaylandObject};

type WlBufferId = u32;
type WlCallbackId = u32;
type WlRegionId = u32;
type RoleObjectId = u32;

// --- State Management Structs ---

/// The role of a surface determines its behavior (e.g., toplevel, popup).
#[derive(Debug, Clone, Copy)]
pub enum SurfaceRole {
    XdgToplevel { toplevel_id: RoleObjectId },
    XdgPopup { popup_id: RoleObjectId },
    // Other roles like cursor, sub-surface would go here.
}

#[derive(Default, Debug)]
struct WlSurfacePendingState {
    buffer: Option<WlBufferId>,
    damage: Vec<(i32, i32, i32, i32)>,
    offset: (i32, i32),
}

#[derive(Debug, Default)]
struct WlSurfaceCurrentState {
    buffer: Option<WlBufferId>,
    texture: Option<Vec<u8>>,
    offset: (i32, i32),
}

/// Implements the wl_surface Wayland object.
pub struct WlSurface {
    pending: WlSurfacePendingState,
    current: WlSurfaceCurrentState,
    role: Option<SurfaceRole>,
}

impl WlSurface {
    pub fn new() -> Self {
        Self {
            pending: WlSurfacePendingState::default(),
            current: WlSurfaceCurrentState::default(),
            role: None,
        }
    }

    // --- Role Management ---
    pub fn set_role(&mut self, role_name: &'static str, role_object_id: RoleObjectId) {
        let new_role = match role_name {
            "xdg_toplevel" => Some(SurfaceRole::XdgToplevel{ toplevel_id: role_object_id }),
            "xdg_popup" => Some(SurfaceRole::XdgPopup{ popup_id: role_object_id }),
            _ => None,
        };
        if self.role.is_none() {
            self.role = new_role;
        } else {
            // This should be a protocol error.
            eprintln!("Error: Attempted to assign a new role to a surface that already has one.");
        }
    }

    pub fn get_role_name(&self) -> Option<String> {
        self.role.map(|r| match r {
            SurfaceRole::XdgToplevel{..} => "xdg_toplevel".to_string(),
            SurfaceRole::XdgPopup{..} => "xdg_popup".to_string(),
        })
    }

    // --- Request Handler Methods ---
    fn handle_attach(&mut self, buffer_id: WlBufferId, x: i32, y: i32) {
        self.pending.buffer = Some(buffer_id);
        self.pending.offset = (x, y);
    }
    fn handle_damage(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.pending.damage.push((x, y, width, height));
    }
    fn handle_frame(&self, _client: &mut WaylandClient, _callback_id: WlCallbackId) {}
    fn handle_commit(&mut self) {
        self.current.buffer = self.pending.buffer;
        self.current.offset = self.pending.offset;
        self.pending = WlSurfacePendingState::default();
    }
}

impl WaylandObject for WlSurface {
    fn on_request(&mut self, client: &mut WaylandClient, request: Request) {
        const DESTROY: u16 = 0;
        const ATTACH: u16 = 1;
        const DAMAGE: u16 = 2;
        const FRAME: u16 = 3;
        const COMMIT: u16 = 6;

        match request.opcode {
            ATTACH => {
                let buffer_id = request.args.get(0).cloned().unwrap_or(0);
                let x = request.args.get(1).cloned().unwrap_or(0) as i32;
                let y = request.args.get(2).cloned().unwrap_or(0) as i32;
                self.handle_attach(buffer_id, x, y);
            }
            DAMAGE => {
                let x = request.args.get(0).cloned().unwrap_or(0) as i32;
                let y = request.args.get(1).cloned().unwrap_or(0) as i32;
                let width = request.args.get(2).cloned().unwrap_or(0) as i32;
                let height = request.args.get(3).cloned().unwrap_or(0) as i32;
                self.handle_damage(x, y, width, height);
            }
            FRAME => {
                if let Some(callback_id) = request.args.get(0) {
                    self.handle_frame(client, *callback_id);
                }
            }
            COMMIT => self.handle_commit(),
            DESTROY => println!("[wl_surface] destroy called (stub)"),
            _ => eprintln!("[wl_surface] Received unknown opcode: {}", request.opcode),
        }
    }
}
