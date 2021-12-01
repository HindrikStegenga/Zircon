use graphyte_engine::platform::*;
use winit::window::Window;

#[derive(Debug)]
pub struct WinitPlatformWindow {
    pub(crate) was_resized: Option<(u32, u32)>,
    pub(crate) window: Window,
    pub(crate) handle: PlatformWindowHandle,
}

unsafe impl HasRawWindowHandle for WinitPlatformWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl PlatformWindow for WinitPlatformWindow {
    fn handle(&self) -> PlatformWindowHandle {
        self.handle
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    fn was_resized(&self) -> Option<(u32, u32)> {
        self.was_resized
    }
}
