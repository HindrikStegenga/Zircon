use ash::extensions::khr::Swapchain;
use ash::*;
use graphyte_engine::PlatformWindowHandle;
use crate::RenderPath;

pub struct WindowRenderTarget {
    window: PlatformWindowHandle,
    surface: vk::SurfaceKHR,
}

pub struct WindowRenderTargetBinding {
    window_render_target: WindowRenderTarget,
    swap_chain: Swapchain,
    render_path: Box<dyn RenderPath>
}

pub enum RenderTarget {
    Window(WindowRenderTargetBinding),
}