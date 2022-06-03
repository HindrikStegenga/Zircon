use utils::defer_drop::*;

use super::PlatformWindowHandle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidOpen {
    pub window: PlatformWindowHandle,
}

#[derive(Clone, Debug)]
pub struct WindowWillClose {
    pub window: PlatformWindowHandle,
    _resource_dropper: Option<DeferDrop>,
}

impl WindowWillClose {
    pub fn new(window: PlatformWindowHandle, resource_to_drop: Option<DeferDrop>) -> Self {
        Self {
            window,
            _resource_dropper: resource_to_drop,
        }
    }
}
unsafe impl Send for WindowWillClose {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WindowDidResize {
    pub window: PlatformWindowHandle,
    pub new_width: u32,
    pub new_height: u32,
}
