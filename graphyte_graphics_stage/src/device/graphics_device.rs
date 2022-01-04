use crate::GraphicsOptions;
use ash::*;

pub struct GraphicsDevice {
    device: Device,
    physical_device: vk::PhysicalDevice,
}

impl GraphicsDevice {
    pub fn select_device(instance: &Instance, options: &GraphicsOptions) -> Option<Self> {
        None
    }
}
