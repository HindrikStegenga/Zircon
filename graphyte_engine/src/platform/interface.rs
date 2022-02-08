use graphyte_asset_library::handles::*;

pub use raw_window_handle::*;
use crate::engine_stages::EngineDidInitInput;
use crate::Platform;

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

pub trait PlatformInterface: std::fmt::Debug {

    fn get_windows(&self) -> Vec<PlatformWindowHandle>;

    fn get_window(&self, handle: PlatformWindowHandle) -> Option<&dyn PlatformWindow>;
    fn get_window_mut(&mut self, handle: PlatformWindowHandle) -> Option<&mut dyn PlatformWindow>;

    fn get_window_handle_by_tag(&self, tag: &str) -> Option<PlatformWindowHandle>;

    fn request_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        tag: Option<String>,
    ) -> Option<&dyn PlatformWindow>;
}
