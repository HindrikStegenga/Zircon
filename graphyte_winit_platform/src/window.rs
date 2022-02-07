use graphyte_engine::platform::*;
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
}
