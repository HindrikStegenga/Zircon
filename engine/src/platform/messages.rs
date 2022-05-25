use super::PlatformWindowHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidOpen {
    pub window: PlatformWindowHandle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidClose {
    pub window: PlatformWindowHandle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidResize {
    pub window: PlatformWindowHandle,
    pub new_width: u32,
    pub new_height: u32,
}
