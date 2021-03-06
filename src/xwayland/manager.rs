//! Global manager for the XWayland server.

use wlroots_sys::wlr_xwayland_surface;

use libc;

use compositor::{compositor_handle, CompositorHandle};

pub trait XWaylandManagerHandler {
    /// Callback that's triggered when the XWayland library is ready.
    fn on_ready(&mut self, CompositorHandle) {}

    // TODO Correct return value (boxed handler)
    fn new_surface(&mut self, CompositorHandle) -> Option<()> {
        None
    }
}

wayland_listener!(XWaylandManager, Box<XWaylandManagerHandler>, [
    on_ready_listener => on_ready_notify: |this: &mut XWaylandManager, _data: *mut libc::c_void,|
    unsafe {
        let manager = &mut this.data;
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        manager.on_ready(compositor);
    };
    new_surface_listener => new_surface_notify: |this: &mut XWaylandManager,
                                                 data: *mut libc::c_void,|
    unsafe {
        let manager = &mut this.data;
        let surface = data as *mut wlr_xwayland_surface;
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        // TODO Pass in the new surface from the data
        manager.new_surface(compositor);
    };
]);
