use magnetar_asset_library::handles::*;

pub use raw_window_handle::*;

pub trait PlatformWindow: HasRawWindowHandle {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn id(&self) -> PlatformWindowHandle;
    //fn raw_window_handle(&self) -> RawWindowHandle;
}

pub type PlatformWindowHandle = Handle<*const dyn PlatformWindow, u16>;

pub trait PlatformInterface: std::fmt::Debug {
    fn get_windows(&self) -> Vec<PlatformWindowHandle>;

    fn get_window(&self, handle: PlatformWindowHandle) -> Option<&dyn PlatformWindow>;
    fn get_window_mut(&mut self, handle: PlatformWindowHandle) -> Option<&mut dyn PlatformWindow>;

    fn request_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
    ) -> Option<&dyn PlatformWindow>;
}
