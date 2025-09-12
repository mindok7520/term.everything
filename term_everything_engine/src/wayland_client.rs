use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::any::Any;

use crate::wayland_object::{Request, WaylandObject};
use crate::protocols::{
    wl_compositor::WlCompositor,
    wl_display::WlDisplay,
    wl_shm::WlShm,
    wl_shm_pool::WlShmPool,
    xdg_wm_base::XdgWmBase,
    wl_surface::WlSurface,
    wl_seat::WlSeat,
};

struct MessageDecoder;
impl MessageDecoder {
    fn consume(&mut self, _buffer: &[u8]) -> Vec<Request> { vec![] }
}

pub struct WaylandClient {
    id: u32,
    pub client_socket_fd: RawFd,
    objects: HashMap<u32, Box<dyn WaylandObject>>,
    message_decoder: MessageDecoder,
    next_serial: u32,
}

const WL_DISPLAY_ID: u32 = 1;
const WL_COMPOSITOR_ID: u32 = 2;
const WL_SHM_ID: u32 = 3;
const XDG_WM_BASE_ID: u32 = 4;
const WL_SEAT_ID: u32 = 5;

impl WaylandClient {
    pub fn new(id: u32, client_socket_fd: RawFd) -> Self {
        let mut client = Self {
            id,
            client_socket_fd,
            objects: HashMap::new(),
            message_decoder: MessageDecoder,
            next_serial: 0,
        };

        // Instantiate and add all the global objects.
        client.add_object(WL_DISPLAY_ID, Box::new(WlDisplay::new()));
        client.add_object(WL_COMPOSITOR_ID, Box::new(WlCompositor::new()));
        client.add_object(WL_SHM_ID, Box::new(WlShm::new()));
        client.add_object(XDG_WM_BASE_ID, Box::new(XdgWmBase::new()));
        client.add_object(WL_SEAT_ID, Box::new(WlSeat::new()));

        client
    }

    pub fn get_id(&self) -> u32 { self.id }

    pub fn add_object(&mut self, id: u32, object: Box<dyn WaylandObject>) {
        println!("[WaylandClient] Adding object with ID: {}", id);
        self.objects.insert(id, object);
    }

    pub fn get_global(&self, id: u32) -> Option<&Box<dyn WaylandObject>> {
        self.objects.get(&id)
    }

    pub fn list_globals(&self) -> Vec<(u32, &'static str, u32)> {
        vec![
            (WL_DISPLAY_ID, "wl_display", 1),
            (WL_COMPOSITOR_ID, "wl_compositor", 4),
            (WL_SHM_ID, "wl_shm", 1),
            (XDG_WM_BASE_ID, "xdg_wm_base", 1),
            (WL_SEAT_ID, "wl_seat", 7),
        ]
    }

    // --- Conceptual methods for object interaction ---
    pub fn get_surface_role(&mut self, surface_id: u32) -> Option<String> {
        self.objects.get_mut(&surface_id)
            .and_then(|s| (s as &mut dyn Any).downcast_mut::<WlSurface>())
            .and_then(|s| s.get_role_name())
    }

    pub fn set_surface_role(&mut self, surface_id: u32, role_object_id: u32, role_name: &'static str) {
         if let Some(surface) = self.objects.get_mut(&surface_id)
            .and_then(|s| (s as &mut dyn Any).downcast_mut::<WlSurface>()) {
            surface.set_role(role_name, role_object_id);
        }
    }

    pub fn signal_buffer_destroyed(&mut self, pool_id: u32, buffer_id: u32) {
        if let Some(pool) = self.objects.get_mut(&pool_id)
            .and_then(|p| (p as &mut dyn Any).downcast_mut::<WlShmPool>()) {
            pool.buffer_destroyed(self, buffer_id);
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        self.next_serial += 1;
        self.next_serial
    }

    pub fn get_client_state_ref(&self) -> u32 { 0 }

    pub async fn main_loop(&mut self) {
        loop {
            println!("[WaylandClient] Finished one loop iteration (stub). Breaking.");
            break;
        }
    }
}
