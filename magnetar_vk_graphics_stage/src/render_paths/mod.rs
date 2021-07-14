use erupt::*;
use magnetar_engine::engine_stages::RenderStageUpdateInput;
use serde::*;
use std::ffi::CString;

pub mod forward;

pub use forward::*;

use crate::{
    components::Camera,
    device::{RenderPathInstance, VkInitializedDevice},
    render_target_bindings::WindowRenderTargetBinding,
    vk_device::VkDevice,
};

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
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
    render_path_type: RenderPathType,
}

impl RenderPathDescriptor {
    pub fn create_instance(
        &self,
        device: &VkInitializedDevice,
        render_target: WindowRenderTargetBinding,
    ) -> Result<RenderPathInstance, (WindowRenderTargetBinding, vk::Result)> {
        Ok(match self.render_path_type() {
            RenderPathType::Forward => {
                RenderPathInstance::Forward(ForwardRenderPath::new(device, render_target)?)
            }
            RenderPathType::Deferred => todo!(),
        })
    }

    pub fn new<T: RenderPath>() -> Self {
        Self {
            name: T::name(),
            required_instance_extensions: T::required_instance_extensions(),
            required_device_extensions: T::required_device_extensions(),
            required_features: T::required_device_features(),
            render_path_type: T::render_path_type(),
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

    /// Get the render path descriptor's render path type.
    pub(crate) fn render_path_type(&self) -> RenderPathType {
        self.render_path_type
    }
}

pub(crate) trait RenderPath {
    fn name() -> String;
    fn render_path_type() -> RenderPathType;
    fn required_instance_extensions() -> Vec<CString>;
    fn required_device_extensions() -> Vec<CString>;
    fn required_device_features() -> vk::PhysicalDeviceFeatures;

    fn render(&mut self, input: &mut RenderStageUpdateInput, camera: &Camera);
}
