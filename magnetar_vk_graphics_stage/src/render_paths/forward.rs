use crate::config::device_features::disabled_device_features;

use super::RenderPath;
use erupt::*;

pub struct ForwardRenderPath {}

impl ForwardRenderPath {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderPath for ForwardRenderPath {
    fn required_instance_extensions() -> Vec<std::ffi::CString> {
        vec![]
    }

    fn required_device_extensions() -> Vec<std::ffi::CString> {
        vec![]
    }

    fn required_device_features() -> vk::PhysicalDeviceFeatures {
        disabled_device_features()
    }

    fn name() -> String {
        "Forward".to_owned()
    }

    fn render_path_type() -> super::RenderPathType {
        super::RenderPathType::Forward
    }
}
