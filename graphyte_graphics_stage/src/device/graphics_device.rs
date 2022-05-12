use crate::device::device_selection::{
    collect_compatible_devices, select_device, DeviceSelectionInfo,
};
use crate::device::queue_types::DeviceQueue;
use crate::{ForwardRenderPath, GraphicsOptions, RenderPath, RenderPathDescriptor};
use ash::*;
use graphyte_utils::tagged_log;
use std::sync::Arc;
use std::vec::Vec;

pub(crate) struct GraphicsDevice {
    instance: Arc<Instance>,
    device: Arc<Device>,
    graphics_queue: DeviceQueue,
    transfer_queues: Vec<DeviceQueue>,
    physical_device: vk::PhysicalDevice,
}

impl GraphicsDevice {
    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn device_arc(&self) -> Arc<Device> {
        Arc::clone(&self.device)
    }
    pub fn graphics_queue(&self) -> &DeviceQueue {
        &self.graphics_queue
    }
    pub fn transfer_queues(&self) -> &[DeviceQueue] {
        &self.transfer_queues
    }
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }
}

pub(crate) struct GraphicsDeviceCreateInfo<'a> {
    pub instance: Arc<Instance>,
    pub options: &'a GraphicsOptions,
}

impl GraphicsDevice {
    pub(crate) fn new(create_info: GraphicsDeviceCreateInfo) -> Option<GraphicsDevice> {
        let instance = create_info.instance.as_ref();

        let render_path_descriptors = [RenderPathDescriptor::new::<ForwardRenderPath>()];

        let compatible_devices = collect_compatible_devices(instance, &render_path_descriptors)?;
        let graphics_device = select_device(create_info.options, compatible_devices)?;
        let creation_result =
            super::setup::setup_device(instance, &graphics_device, create_info.options)?;

        Self {
            instance: Arc::clone(&create_info.instance),
            device: Arc::new(creation_result.device),
            graphics_queue: creation_result.graphics_queue,
            transfer_queues: creation_result.transfer_queues,
            physical_device: graphics_device.device,
        }
        .into()
    }
}

impl Drop for GraphicsDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
