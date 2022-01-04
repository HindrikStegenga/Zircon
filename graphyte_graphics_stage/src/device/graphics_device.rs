use std::sync::Arc;
use crate::{ForwardRenderPath, GraphicsOptions, RenderPath, RenderPathDescriptor};
use ash::*;
use crate::device::device_selection::{collect_compatible_devices, select_device};

pub(crate) struct GraphicsDevice {
    instance: Arc<Instance>,
    //device: Device,
    physical_device: vk::PhysicalDevice,
}

pub(crate) struct GraphicsDeviceCreateInfo<'a> {
    pub instance: Arc<Instance>,
    pub options: &'a GraphicsOptions
}

impl GraphicsDevice {
    pub(crate) fn new(create_info: GraphicsDeviceCreateInfo) -> Option<GraphicsDevice> {
        let instance = create_info.instance.as_ref();

        let render_path_descriptors = [
            RenderPathDescriptor::new::<ForwardRenderPath>()
        ];

        let compatible_devices = collect_compatible_devices(instance, &render_path_descriptors)?;
        let graphics_device = select_device(create_info.options, compatible_devices)?;



        Self {
            instance: Arc::clone(&create_info.instance),
            //device: (),
            physical_device: graphics_device.device
        }.into()
    }
}