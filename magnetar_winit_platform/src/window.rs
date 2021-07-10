use magnetar_engine::*;
use winit::window::Window;

#[derive(Debug)]
pub struct WinitPlatformWindow {
    pub(crate) window: Window,
    pub(crate) id: PlatformWindowHandle,
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
