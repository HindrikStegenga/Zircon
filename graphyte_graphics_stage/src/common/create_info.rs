use std::sync::Arc;
use graphyte_engine::{ApplicationInfo, AssetSystem, PlatformInterface};
use crate::GraphicsBackend;

pub struct GraphicsStageCreateInfo<'a> {
    pub preferred_api: String,
    pub application_info: ApplicationInfo,
    pub platform: &'a mut dyn PlatformInterface,
    pub asset_system: Arc<AssetSystem>,
    #[cfg(feature = "vulkan_api")]
    pub vulkan: <crate::vulkan::VulkanRenderBackend as GraphicsBackend>::GraphicsOptions,
    #[cfg(feature = "open_gl_api")]
    pub open_gl: <crate::open_gl::OpenGLRenderBackend as GraphicsBackend>::GraphicsOptions,
    #[cfg(feature = "metal_api")]
    pub metal: <crate::metal::MetalRenderBackend as GraphicsBackend>::GraphicsOptions,
}