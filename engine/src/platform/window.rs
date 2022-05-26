use asset_library::handles::*;
pub use raw_window_handle::*;

pub trait PlatformWindow: HasRawWindowHandle {
    fn tag(&self) -> Option<&str>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn handle(&self) -> PlatformWindowHandle;
    fn raw_platform_handle(&self) -> RawPlatformWindow {
        RawPlatformWindow::new(self.handle(), Self::raw_window_handle(&self))
    }
}

#[derive(Copy, Clone)]
pub struct RawPlatformWindow {
    handle: PlatformWindowHandle,
    raw_window_handle: RawWindowHandle,
}

impl RawPlatformWindow {
    pub fn new(handle: PlatformWindowHandle, raw_window_handle: RawWindowHandle) -> Self {
        Self {
            handle,
            raw_window_handle,
        }
    }
    pub fn handle(&self) -> PlatformWindowHandle {
        self.handle
    }
    pub fn raw_handle(&self) -> RawWindowHandle {
        self.raw_window_handle
    }
}

unsafe impl HasRawWindowHandle for RawPlatformWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw_handle()
    }
}

pub type PlatformWindowHandle = Handle<*const dyn PlatformWindow, u16>;
