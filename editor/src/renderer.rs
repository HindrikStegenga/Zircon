use std::sync::Arc;

use ash::*;
use graphics::*;

use crate::EguiRenderPlugin;

pub struct EguiRenderer {
    device: Arc<Device>,
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
}

impl Drop for EguiRenderer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None)
        }
    }
}

impl EguiRenderer {
    pub(crate) fn new(
        instance: &ash::Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &mut dyn engine::PlatformInterface,
        window_render_target: &WindowRenderTarget,
        swap_chain: &SwapChain,
        options: &GraphicsOptions,
    ) -> Option<Self> {
        let pool_sizes = vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1024)
            .build();

        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::builder()
            .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(1024)
            .pool_sizes(&[pool_sizes])
            .build();

        let descriptor_pool = unsafe {
            device
                .device()
                .create_descriptor_pool(&descriptor_pool_create_info, None)
        }
        .ok()?;

        let descriptor_sets = vec![];

        Self {
            device: device.device_arc(),
            descriptor_pool,
            descriptor_sets,
        }
        .into()
    }

    fn pre_render(&mut self, info: RenderPluginRenderInfo) {}

    fn post_render(&mut self, info: RenderPluginRenderInfo) {}
}
