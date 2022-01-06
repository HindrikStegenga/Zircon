use super::WindowRenderTarget;
use crate::RenderPath;
use ash::extensions::khr::{Surface, Swapchain};
use ash::*;
use graphyte_engine::{PlatformWindow, PlatformWindowHandle};
use std::sync::Arc;

pub struct WindowRenderTargetBinding {
    window_render_target: WindowRenderTarget,
    swap_chain: Swapchain,
    render_path: Box<dyn RenderPath>,
}

pub enum RenderTarget {
    Window(WindowRenderTargetBinding),
}
