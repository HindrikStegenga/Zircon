use super::WindowRenderTarget;
use crate::render_target::WindowRenderTargetBinding;
use crate::RenderPath;
use ash::extensions::khr::{Surface, Swapchain};
use ash::*;
use graphyte_engine::{PlatformWindow, PlatformWindowHandle};
use std::sync::Arc;

pub(crate) enum RenderTarget {
    Window(WindowRenderTargetBinding),
}
