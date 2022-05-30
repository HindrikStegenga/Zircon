use engine::{handles::Handle, platform::*};
use winit::window::Window;

#[derive(Debug)]
pub struct WinitPlatformWindow {
    pub(crate) window: Window,
    pub(crate) handle: PlatformWindowHandle,
    pub(crate) tag: Option<String>,
}

unsafe impl HasRawWindowHandle for WinitPlatformWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl WinitPlatformWindow {
    /// Get a reference to the winit platform window's window.
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get a mutable reference to the winit platform window's window.
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Get a reference to the winit platform window's handle.
    pub fn handle(&self) -> Handle<*const dyn PlatformWindow, u16> {
        self.handle
    }

    /// Get a reference to the winit platform window's tag.
    pub fn tag(&self) -> Option<&String> {
        self.tag.as_ref()
    }
}

impl PlatformWindow for WinitPlatformWindow {
    fn tag(&self) -> Option<&str> {
        return if let Some(v) = &self.tag {
            Some(v.as_str())
        } else {
            None
        };
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    fn handle(&self) -> PlatformWindowHandle {
        self.handle
    }

    fn pixels_per_point(&self) -> f32 {
        self.window.scale_factor() as f32
    }
}
