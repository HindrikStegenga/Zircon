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
pub struct RenderPathDescriptor {
    pub(crate) name: fn() -> String,
    pub(crate) required_instance_extensions: fn() -> Vec<CString>,
    pub(crate) required_device_extensions: fn() -> Vec<CString>,
    pub(crate) required_features: fn() -> vk::PhysicalDeviceFeatures,
}

pub trait RenderPath {
    fn name() -> String;
    fn required_instance_extensions() -> Vec<CString>;
    fn required_device_extensions() -> Vec<CString>;
    fn required_device_features() -> vk::PhysicalDeviceFeatures;
}

impl RenderPathDescriptor {
    pub fn from_path<T: RenderPath>() -> Self {
        Self {
            name: T::name,
            required_instance_extensions: T::required_instance_extensions,
            required_device_extensions: T::required_device_extensions,
            required_features: T::required_device_features,
        }
    }
}
