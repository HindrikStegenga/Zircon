use crate::device::device_selection::{collect_compatible_devices, DeviceSelectionInfo, select_device};
use crate::{ForwardRenderPath, GraphicsOptions, RenderPath, RenderPathDescriptor};
use ash::*;
use std::sync::Arc;
use std::vec::Vec;
use crate::device::queue_types::DeviceQueue;

pub(crate) struct GraphicsDevice {
    instance: Arc<Instance>,
    device: Device,
    graphics_queue: DeviceQueue,
    transfer_queues: Vec<DeviceQueue>,
    physical_device: vk::PhysicalDevice,
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
        let creation_result = super::setup::setup_device(instance, &graphics_device, create_info.options)?;

        Self {
            instance: Arc::clone(&create_info.instance),
            device: creation_result.device,
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