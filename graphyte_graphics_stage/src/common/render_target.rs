use graphyte_engine::PlatformWindowHandle;
use ash::*;
use ash::extensions::khr::Swapchain;

pub struct WindowRenderTarget {
    window: PlatformWindowHandle,
    surface: vk::SurfaceKHR
}

pub struct WindowRenderTargetBinding {
    // TODO: scissor rectangles and what not.
    swap_chain: Swapchain,
}

pub enum RenderTarget {
    Window(WindowRenderTargetBinding)
}