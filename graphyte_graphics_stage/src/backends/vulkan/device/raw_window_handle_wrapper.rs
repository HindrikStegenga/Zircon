use graphyte_engine::platform::{HasRawWindowHandle, RawWindowHandle};
pub(crate) struct RawWindowHandleWrapper {
    handle: RawWindowHandle,
}
unsafe impl HasRawWindowHandle for RawWindowHandleWrapper {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle
    }
}
impl From<RawWindowHandle> for RawWindowHandleWrapper {
    fn from(handle: RawWindowHandle) -> Self {
        Self { handle }
    }
}
