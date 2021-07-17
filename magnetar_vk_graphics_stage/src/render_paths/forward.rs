use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    components::Camera,
    config::device_features::disabled_device_features,
    device::{commandpool::VkCommandPool, shader::VkShaderModule, VkInitializedDevice, VkQueue},
    render_target_bindings::WindowRenderTargetBinding,
    vk_device::VkDevice,
};

use super::RenderPath;
use erupt::*;
use magnetar_engine::{engine_stages::RenderStageUpdateInput, *};

pub(crate) struct ForwardRenderPath {
    command_buffers: Vec<vk::CommandBuffer>,
    frame_buffers: Vec<vk::Framebuffer>,
    render_pass: vk::RenderPass,
    graphics_command_pool: VkCommandPool,
    render_target: WindowRenderTargetBinding,
    graphics_queue: VkQueue,
    device: VkDevice,
}

impl ForwardRenderPath {
    pub fn new(
        asset_system: Arc<AssetSystem>,
        device: &VkInitializedDevice,
        render_target: WindowRenderTargetBinding,
    ) -> Result<Self, (WindowRenderTargetBinding, vk::Result)> {
        let device_handle: VkDevice = device.deref().clone();
        let graphics_queue = device.graphics_queue().clone();

        let mut graphics_command_pool = match VkCommandPool::new(
            device_handle.clone(),
            graphics_queue.family_index,
            true,
            false,
        ) {
            Ok(v) => v,
            Err(e) => return Err((render_target, e)),
        };
        let render_pass =
            match Self::init_default_render_pass(device, render_target.surface_format()) {
                Ok(v) => v,
                Err(e) => return Err((render_target, e)),
            };

        let frame_buffers =
            match Self::init_default_frame_buffers(device, &render_target, render_pass) {
                Ok(v) => v,
                Err(e) => return Err((render_target, e)),
            };

        tagged_success!(
            "VkGraphics Stage",
            "Successfully created Forward render path."
        );

        Ok(Self {
            command_buffers: (0..render_target.image_count())
                .into_iter()
                .map(|_| {
                    graphics_command_pool
                        .allocate_primary_command_buffer()
                        .unwrap()
                })
                .collect(),
            graphics_command_pool,
            graphics_queue,
            render_pass,
            device: device_handle,
            render_target,
            frame_buffers,
        })
    }

    pub fn init_default_render_pass(
        device: &VkDevice,
        format: vk::SurfaceFormatKHR,
    ) -> Result<vk::RenderPass, vk::Result> {
        let color_attach = [vk::AttachmentDescriptionBuilder::new()
            .format(format.format)
            .samples(vk::SampleCountFlagBits::_1)
            .load_op(vk::AttachmentLoadOp::DONT_CARE)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)];

        let color_attach_ref = [vk::AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpass = [vk::SubpassDescriptionBuilder::new()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attach_ref)];

        let create_info = vk::RenderPassCreateInfoBuilder::new()
            .attachments(&color_attach)
            .subpasses(&subpass);

        Ok(unsafe { device.create_render_pass(&create_info, None).result()? })
    }

    pub fn init_default_frame_buffers(
        device: &VkDevice,
        render_target: &WindowRenderTargetBinding,
        render_pass: vk::RenderPass,
    ) -> Result<Vec<vk::Framebuffer>, vk::Result> {
        let mut framebuffers = Vec::with_capacity(render_target.image_count() as usize);
        for i in 0..render_target.image_count() {
            let attach = [render_target.image_views()[i as usize]];
            let create_info = vk::FramebufferCreateInfoBuilder::new()
                .render_pass(render_pass)
                .attachments(&attach)
                .width(render_target.surface_extent().width)
                .height(render_target.surface_extent().height)
                .layers(1);
            let buf = unsafe {
                match device.create_framebuffer(&create_info, None).result() {
                    Ok(v) => v,
                    Err(e) => {
                        framebuffers.iter().for_each(|f| {
                            device.destroy_framebuffer(Some(*f), None);
                        });
                        return Err(e);
                    }
                }
            };
            framebuffers.push(buf);
        }
        Ok(framebuffers)
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

    fn render(&mut self, input: &mut RenderStageUpdateInput, camera: &Camera) {}
}

impl Drop for ForwardRenderPath {
    fn drop(&mut self) {
        unsafe {
            self.device
                .queue_wait_idle(self.graphics_queue.queue)
                .result()
                .unwrap();
            self.frame_buffers.iter().for_each(|f| {
                self.device.destroy_framebuffer(Some(*f), None);
            });
            self.frame_buffers.clear();
            self.device
                .destroy_render_pass(Some(self.render_pass), None)
        };
    }
}
