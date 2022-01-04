use std::ffi::{CStr, CString};
use ash::*;
use crate::{ForwardRenderPath, GraphicsOptions};

#[derive(Clone)]
pub(crate) struct RenderPathDescriptor {
    required_features: vk::PhysicalDeviceFeatures,
    instantiate_fn: fn(options: GraphicsOptions) -> Option<Box<dyn RenderPath>>,
    identifier: CString,
}

impl RenderPathDescriptor {
    pub fn required_features(&self) -> vk::PhysicalDeviceFeatures {
        self.required_features
    }
    pub fn instantiate_fn(&self) -> fn(GraphicsOptions) -> Option<Box<dyn RenderPath>> {
        self.instantiate_fn
    }
}

impl RenderPathDescriptor {
    pub fn new<T: RenderPath + Sized + 'static>() -> Self {
        Self {
            required_features: T::required_device_features(),
            instantiate_fn: |options| {
                return if let Some(value) = T::instantiate(options) {
                    Some(Box::from(value))
                } else { None }
            },
            identifier: T::render_path_identifier()
        }
    }
}

pub(crate) trait RenderPath {
    fn render_path_identifier() -> CString
    where
        Self: Sized;

    fn required_device_features() -> vk::PhysicalDeviceFeatures
    where
        Self: Sized;
    fn instantiate(options: GraphicsOptions) -> Option<Self>
    where
        Self: Sized;

    fn render(&mut self);
}
