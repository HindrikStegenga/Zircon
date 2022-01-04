use std::ffi::{CStr, CString};
use crate::{GraphicsOptions, RenderPath, RenderPathCreateInfo};
use ash::vk::PhysicalDeviceFeatures;
use ash::*;

pub struct ForwardRenderPath {}

impl RenderPath for ForwardRenderPath {
    fn render_path_identifier() -> CString where Self: Sized {
        CString::new(b"Forward" as &[u8]).unwrap()
    }

    fn required_device_features() -> vk::PhysicalDeviceFeatures
    where
        Self: Sized,
    {
        vk::PhysicalDeviceFeatures::default()
    }

    fn instantiate(create_info: RenderPathCreateInfo) -> Option<Self> where Self: Sized {
        Self {}.into()
    }

    fn render(&mut self) {}
}