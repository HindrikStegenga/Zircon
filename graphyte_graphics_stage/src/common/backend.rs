use std::sync::Arc;
use graphyte_engine::*;
use graphyte_engine::engine_stages::{RenderStageUpdateInput, UpdateStageUpdateInput};
use graphyte_engine::message_bus::MessageRegisterer;
use crate::{GraphicsStage, GraphicsStageCreateInfo};

pub(crate) enum GraphicsBackendContainer {
    #[cfg(feature = "metal_api")]
    Metal(crate::metal::MetalRenderBackend),
    #[cfg(feature = "vulkan_api")]
    Vulkan(crate::vulkan::VulkanRenderBackend),
    #[cfg(feature = "open_gl_api")]
    OpenGL(crate::open_gl::OpenGLRenderBackend),
}

impl GraphicsBackendContainer {

    pub fn new(mut create_info: GraphicsStageCreateInfo<'_>) -> Option<Self> {
        if let Some(_preferred_api_string) = create_info.preferred_api {
            todo!()
        } else {
            #[cfg(feature = "metal_api")]
            if let Ok(backend) = crate::metal::MetalRenderBackend::new(GraphicsBackendCreateInfo::<<crate::metal::MetalRenderBackend as GraphicsBackend>::GraphicsOptions> {
                graphics_options: (),
                application_info: create_info.application_info.clone(),
                asset_system: Arc::clone(&create_info.asset_system),
                platform_interface: &mut create_info.platform
            }) {
                Some(Self::Metal(backend))
            }
            #[cfg(feature = "vulkan_api")]
            if let Ok(backend) = crate::vulkan::VulkanRenderBackend::new(GraphicsBackendCreateInfo::<<crate::vulkan::VulkanRenderBackend as GraphicsBackend>::GraphicsOptions> {
                graphics_options: create_info.vulkan,
                application_info: create_info.application_info.clone(),
                asset_system: Arc::clone(&create_info.asset_system),
                platform_interface: create_info.platform
            }) {
                return Some(Self::Vulkan(backend));
            }
            #[cfg(feature = "open_gl_api")]
            if let Ok(backend) = crate::open_gl::OpenGLRenderBackend::new(GraphicsBackendCreateInfo::<<crate::open_gl::OpenGLRenderBackend as GraphicsBackend>::GraphicsOptions> {
                graphics_options: create_info.open_gl,
                application_info: create_info.application_info,
                asset_system: Arc::clone(&create_info.asset_system),
                platform_interface: create_info.platform
            }) {
                return Some(Self::OpenGL(backend));
            }
            None
        }
    }
}

pub struct GraphicsBackendCreateInfo<'a, T> {
    pub graphics_options: T,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetSystem>,
    pub platform_interface: &'a mut dyn PlatformInterface,
}

pub trait GraphicsBackend : Sized {
    const API_IDENTIFIER: &'static str;
    type GraphicsOptions;
    type ErrorType;

    fn new(create_info: GraphicsBackendCreateInfo<'_, Self::GraphicsOptions>) -> Result<Self, Self::ErrorType>;

    fn pre_update(input: UpdateStageUpdateInput) -> EngineUpdateResult
    { EngineUpdateResult::Ok }
    fn post_update(input: UpdateStageUpdateInput) -> EngineUpdateResult
    { EngineUpdateResult::Ok }
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}