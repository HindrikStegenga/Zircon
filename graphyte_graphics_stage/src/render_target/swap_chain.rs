use ash::*;
use ash_window::*;
use graphyte_engine::{HasRawWindowHandle, PlatformWindow, RawWindowHandle};

pub(crate) struct SwapChain {}

pub fn get_vulkan_surface(
    entry: &Entry,
    instance: &Instance,
    window_handle: &dyn HasRawWindowHandle,
) -> Option<vk::SurfaceKHR> {
    unsafe {
        ash_window::create_surface(entry, instance, &window_handle, None)
            .ok()?
            .into()
    }
}
