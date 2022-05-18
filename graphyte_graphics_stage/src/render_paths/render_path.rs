use crate::{
    render_target::{SwapChain, WindowRenderTarget},
    Camera, ForwardRenderPath, GraphicsDevice, GraphicsOptions,
};
use ash::*;
use graphyte_engine::RenderStageUpdateInput;
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};

#[repr(u8)]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum RenderPathType {
    Forward = 0,
}

#[derive(Clone)]
pub(crate) struct RenderPathDescriptor {
    required_features: vk::PhysicalDeviceFeatures,
    required_extensions: Vec<CString>,
    instantiate_fn: fn(create_info: RenderPathCreateInfo) -> Option<Box<dyn RenderPath>>,
    identifier: CString,
}

impl RenderPathDescriptor {
    pub fn required_extensions(&self) -> &Vec<CString> {
        &self.required_extensions
    }
    pub fn required_features(&self) -> vk::PhysicalDeviceFeatures {
        self.required_features
    }
    pub fn instantiate_fn(&self) -> fn(RenderPathCreateInfo) -> Option<Box<dyn RenderPath>> {
        self.instantiate_fn
    }
    pub fn identifier(&self) -> &CStr {
        &self.identifier
    }
}

pub(crate) struct RenderPathCreateInfo<'a> {
    pub options: &'a GraphicsOptions,
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
            instantiate_fn: |create_info| {
                return if let Some(value) = T::instantiate(create_info) {
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
    fn instantiate(create_info: RenderPathCreateInfo) -> Option<Self>
    where
        Self: Sized;

    fn render(
        &mut self,
        camera: &Camera,
        swap_chain: &mut SwapChain,
        window_render_target: &mut WindowRenderTarget,
        device: &GraphicsDevice,
        input: &mut RenderStageUpdateInput,
    ) -> bool;
}
