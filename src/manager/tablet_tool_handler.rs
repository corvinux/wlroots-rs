//! Handler for tablet tools

use {InputDevice, TabletTool, TabletToolHandle};
use compositor::{compositor_handle, CompositorHandle};
use events::tablet_tool_events::{AxisEvent, ButtonEvent, ProximityEvent, TipEvent};
use libc;

pub trait TabletToolHandler {
    /// Callback that is triggered when an axis event fires
    fn on_axis(&mut self, CompositorHandle, TabletToolHandle, &AxisEvent) {}

    /// Callback that is triggered when a table tool is brought close to the
    /// input source.
    fn on_proximity(&mut self, CompositorHandle, TabletToolHandle, &ProximityEvent) {}

    /// Callback that is triggered when a table tool's tip touches the input
    /// source.
    fn on_tip(&mut self, CompositorHandle, TabletToolHandle, &TipEvent) {}

    /// Callback that is triggered when a button is pressed on the tablet tool.
    fn on_button(&mut self, CompositorHandle, TabletToolHandle, &ButtonEvent) {}
}

wayland_listener!(TabletToolWrapper, (TabletTool, Box<TabletToolHandler>), [
    axis_listener => axis_notify: |this: &mut TabletToolWrapper, data: *mut libc::c_void,| unsafe {
        let (ref tool, ref mut handler) = this.data;
        let event = AxisEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_axis(compositor,
                        tool.weak_reference(),
                        &event);
    };
    proximity_listener => proximity_notify: |this: &mut TabletToolWrapper,
    data: *mut libc::c_void,|
    unsafe {
        let (ref tool, ref mut handler) = this.data;
        let event = ProximityEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_proximity(compositor,
                             tool.weak_reference(),
                             &event);
    };
    tip_listener => tip_notify: |this: &mut TabletToolWrapper, data: *mut libc::c_void,| unsafe {
        let (ref tool, ref mut handler) = this.data;
        let event = TipEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_tip(compositor,
                       tool.weak_reference(),
                       &event);
    };
    button_listener => button_notify: |this: &mut TabletToolWrapper, data: *mut libc::c_void,|
    unsafe {
        let (ref tool, ref mut handler) = this.data;
        let event = ButtonEvent::from_ptr(data as *mut _);
        let compositor = match compositor_handle() {
            Some(handle) => handle,
            None => return
        };

        handler.on_button(compositor,
                          tool.weak_reference(),
                          &event);
    };
]);

impl TabletToolWrapper {
    pub(crate) fn input_device(&self) -> &InputDevice {
        self.data.0.input_device()
    }

    pub fn tablet_tool(&mut self) -> TabletToolHandle {
        self.data.0.weak_reference()
    }
}
