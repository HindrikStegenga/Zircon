use ash::extensions::khr::Swapchain;
use ash::*;
use graphyte_engine::PlatformWindowHandle;

pub struct WindowRenderTarget {
    window: PlatformWindowHandle,
    surface: vk::SurfaceKHR,
}

pub struct WindowRenderTargetBinding {
    // TODO: scissor rectangles and what not.
    swap_chain: Swapchain,
}

pub enum RenderTarget {
    Window(WindowRenderTargetBinding),
}
