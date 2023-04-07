use crate::{
    render_target::{AcquiredFrameInfo, SwapChain, WindowRenderTarget},
    Camera, GraphicsDevice, GraphicsOptions,
};
use ash::*;
use assets::AssetCache;
use engine::RenderStageUpdateInput;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

#[repr(u8)]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RenderPathType {
    Forward = 0,
}

#[derive(Clone)]
pub(crate) struct RenderPathDescriptor {
    required_features: vk::PhysicalDeviceFeatures,
    required_extensions: Vec<CString>,
    new_fn: fn(create_info: RenderPathCreateInfo) -> Option<Box<dyn RenderPath>>,
    identifier: CString,
}

impl RenderPathDescriptor {
    pub fn required_extensions(&self) -> &Vec<CString> {
        &self.required_extensions
    }
    pub fn required_features(&self) -> vk::PhysicalDeviceFeatures {
        self.required_features
    }
    pub fn new_fn(&self) -> fn(RenderPathCreateInfo) -> Option<Box<dyn RenderPath>> {
        self.new_fn
    }
    pub fn identifier(&self) -> &CStr {
        &self.identifier
    }
}

pub(crate) struct RenderPathCreateInfo<'a> {
    pub options: &'a GraphicsOptions,
    pub asset_cache: Arc<AssetCache>,
    pub graphics_device: &'a GraphicsDevice,
    pub camera: &'a Camera,
    pub swap_chain: &'a mut SwapChain,
    pub window_render_target: &'a mut WindowRenderTarget,
}

impl RenderPathDescriptor {
    pub fn new<T: RenderPath + Sized + 'static>() -> Self {
        Self {
            required_features: T::required_device_features(),
            required_extensions: T::required_device_extensions(),
            new_fn: |create_info| {
                return if let Some(value) = T::new(create_info) {
                    Some(Box::from(value))
                } else {
                    None
                };
            },
            identifier: T::render_path_identifier(),
        }
    }
}

pub(crate) trait RenderPath {
    fn render_path_identifier() -> CString
    where
        Self: Sized;

    fn required_device_extensions() -> Vec<CString>
    where
        Self: Sized;

    fn required_device_features() -> vk::PhysicalDeviceFeatures
    where
        Self: Sized;
    fn new(create_info: RenderPathCreateInfo) -> Option<Self>
    where
        Self: Sized;

    // This is triggered right before the swap chain will be resized.
    // This allows the render path to clean up resources that were associated with the swap chain.
    fn swapchain_will_be_resized(&mut self) -> bool;

    // Executed after the swapchain size has been adjusted.
    // This allows the render path to set up required resources.
    fn swapchain_did_resize(
        &mut self,
        camera: &Camera,
        swap_chain: &mut SwapChain,
        window_render_target: &mut WindowRenderTarget,
        device: &GraphicsDevice,
    ) -> bool;

    fn render(
        &mut self,
        camera: &Camera,
        info: &AcquiredFrameInfo,
        window_render_target: &mut WindowRenderTarget,
        device: &GraphicsDevice,
        input: &mut RenderStageUpdateInput,
    ) -> bool;
}
