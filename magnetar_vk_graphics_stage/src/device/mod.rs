use std::ffi::CString;

use crate::{
    render_paths::{ForwardRenderPath, RenderPath},
    render_target_output::RenderTargetOutput,
};
use erupt::*;

pub mod bindings;
pub mod device_selection;
pub mod raw_window_handle_wrapper;
pub use bindings::*;
pub use device_selection::*;
use raw_window_handle_wrapper::*;

pub struct VkDevice {}

impl VkDevice {
    pub fn new(
        instance: &InstanceLoader,
        physical_device: vk::PhysicalDevice,
        required_extension_names: Vec<CString>,
        required_features: vk::PhysicalDeviceFeatures,
        device_queue_designation: DeviceQueueFamilyDesignation,
    ) -> Self {
        Self {}
    }
}
