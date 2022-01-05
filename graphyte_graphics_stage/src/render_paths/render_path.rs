use crate::{ForwardRenderPath, GraphicsOptions};
use ash::*;
use std::ffi::{CStr, CString};

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

    fn render(&mut self);
}
