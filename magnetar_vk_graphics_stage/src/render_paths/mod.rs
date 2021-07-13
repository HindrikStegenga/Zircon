use erupt::*;
use serde::*;
use std::ffi::CString;

pub mod forward;

pub use forward::*;

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize)]
pub enum RenderPathType {
    Forward = 0,
    Deferred = 1,
}

#[derive(Debug, Clone)]
pub(crate) struct RenderPathDescriptor {
    name: String,
    required_instance_extensions: Vec<CString>,
    required_device_extensions: Vec<CString>,
    required_features: vk::PhysicalDeviceFeatures,
}

impl RenderPathDescriptor {
    pub fn new<T: RenderPath>() -> Self {
        Self {
            name: T::name(),
            required_instance_extensions: T::required_instance_extensions(),
            required_device_extensions: T::required_device_extensions(),
            required_features: T::required_device_features(),
        }
    }

    /// Get a reference to the render path descriptor's name.
    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get a reference to the render path descriptor's required instance extensions.
    pub(crate) fn required_instance_extensions(&self) -> &[CString] {
        self.required_instance_extensions.as_slice()
    }

    /// Get a reference to the render path descriptor's required device extensions.
    pub(crate) fn required_device_extensions(&self) -> &[CString] {
        self.required_device_extensions.as_slice()
    }

    /// Get a reference to the render path descriptor's required features.
    pub(crate) fn required_features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.required_features
    }
}

pub trait RenderPath {
    fn name() -> String;
    fn required_instance_extensions() -> Vec<CString>;
    fn required_device_extensions() -> Vec<CString>;
    fn required_device_features() -> vk::PhysicalDeviceFeatures;
}
