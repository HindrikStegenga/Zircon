use super::PlatformWindowHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidOpen {
    window: PlatformWindowHandle
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidClose {
    window: PlatformWindowHandle
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidResize {
    window: PlatformWindowHandle,
    new_width: u32,
    new_height: u32
}