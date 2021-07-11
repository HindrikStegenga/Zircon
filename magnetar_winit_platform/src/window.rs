use magnetar_engine::platform::*;
use winit::window::Window;

#[derive(Debug)]
pub struct WinitPlatformWindow {
    pub(crate) window: Window,
    pub(crate) id: PlatformWindowHandle,
}

unsafe impl HasRawWindowHandle for WinitPlatformWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl PlatformWindow for WinitPlatformWindow {
    fn id(&self) -> PlatformWindowHandle {
        self.id
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }
}
