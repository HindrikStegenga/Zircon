use std::{ops::Deref, sync::Arc};

use crate::{
    components::Camera,
    config::device_features::disabled_device_features,
    device::{commandpool::VkCommandPool, shader::VkShaderModule, VkInitializedDevice, VkQueue},
    render_target_bindings::PresentResult,
    render_target_bindings::WindowRenderTargetBinding,
    vk_device::VkDevice,
};

use super::RenderPath;
use erupt::*;
use graphyte_engine::{engine_stages::RenderStageUpdateInput, *};

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
            .load_op(vk::AttachmentLoadOp::CLEAR)
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

        // let dependencies = [vk::SubpassDependencyBuilder::new()
        //     .src_subpass(vk::SUBPASS_EXTERNAL)
        //     .dst_subpass(0)
        //     .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        //     .src_access_mask(vk::AccessFlags::empty())
        //     .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        //     .dst_access_mask(vk::AccessFlags::empty())
        // ];

        let create_info = vk::RenderPassCreateInfoBuilder::new()
            .attachments(&color_attach)
            .subpasses(&subpass);
        //.dependencies(&dependencies);

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

    fn on_resized_render_target(&mut self, _width: u32, _height: u32) -> Result<(), vk::Result> {
        let graphics_command_pool = match VkCommandPool::new(
            self.device.clone(),
            self.graphics_queue.family_index,
            true,
            false,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        self.frame_buffers.iter().for_each(|f| unsafe {
            self.device.destroy_framebuffer(Some(*f), None);
        });
        self.frame_buffers.clear();
        self.graphics_command_pool.reset()?;
        unsafe {
            self.device
                .destroy_render_pass(Some(self.render_pass), None);
        }
        self.render_pass =
            match Self::init_default_render_pass(&self.device, self.render_target.surface_format())
            {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        self.frame_buffers = match Self::init_default_frame_buffers(
            &self.device,
            &self.render_target,
            self.render_pass,
        ) {
            Ok(v) => v,
            Err(e) => {
                unsafe {
                    self.device
                        .destroy_render_pass(Some(self.render_pass), None);
                }
                return Err(e);
            }
        };

        self.graphics_command_pool = graphics_command_pool;
        self.command_buffers = (0..self.render_target.image_count())
            .into_iter()
            .map(|_| {
                self.graphics_command_pool
                    .allocate_primary_command_buffer()
                    .unwrap()
            })
            .collect();

        Ok(())
    }

    fn render(
        &mut self,
        _input: &mut RenderStageUpdateInput,
        camera: &Camera,
    ) -> Result<PresentResult, vk::Result> {
        let image_info = match self.render_target.acquire_next_image() {
            Ok(v) => match v {
                crate::render_target_bindings::PresentImageInfo::Acquired(e) => e,
                crate::render_target_bindings::PresentImageInfo::SubOptimal(e) => e,
                crate::render_target_bindings::PresentImageInfo::OutOfDate => {
                    return Ok(PresentResult::OutOfDate)
                }
            },
            Err(e) => {
                warn!("Image acquire error: {}", e);
                return Err(e);
            }
        };

        let command_buffer = self.command_buffers[image_info.image_index as usize];
        unsafe {
            self.device
                .reset_command_buffer(command_buffer, None)
                .result()
                .unwrap()
        };

        unsafe {
            let begin_info = vk::CommandBufferBeginInfoBuilder::new()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            self.device
                .begin_command_buffer(command_buffer, &begin_info)
                .result()
                .unwrap();

            let mut clear_values = [vk::ClearValue::default()];
            clear_values[0].color = vk::ClearColorValue {
                float32: [1.0f32, 0.0f32, 0.0f32, 1.0f32],
            };

            let mut render_area = vk::Rect2D::default();
            render_area.offset.x = 0;
            render_area.offset.y = 0;
            render_area.extent = self.render_target.surface_extent();
            let render_pass_begin = vk::RenderPassBeginInfoBuilder::new()
                .clear_values(&clear_values)
                .render_pass(self.render_pass)
                .render_area(render_area)
                .framebuffer(self.frame_buffers[image_info.image_index as usize]);

            self.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin,
                vk::SubpassContents::INLINE,
            );

            self.device.cmd_end_render_pass(command_buffer);
            self.device
                .end_command_buffer(command_buffer)
                .result()
                .unwrap();
        };
        let cbufs = [command_buffer];
        let wait_semas = [image_info.image_available_semaphore];
        let signal_semas = [image_info.render_finished_semaphore];

        let submit_info = [vk::SubmitInfoBuilder::new()
            .command_buffers(&cbufs)
            .wait_semaphores(&wait_semas)
            .signal_semaphores(&signal_semas)
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])];

        if let Err(e) = unsafe {
            self.device
                .reset_fences(&[image_info.cmd_submission_fence])
                .result()
        } {
            tagged_warn!("VkGraphics Stage", "Unable to reset fence: {:#?}", e);
            return Err(e);
        };

        if let Err(e) = unsafe {
            self.device
                .queue_submit(
                    self.graphics_queue.queue,
                    &submit_info,
                    Some(image_info.cmd_submission_fence),
                )
                .result()
        } {
            tagged_warn!("VkGraphics Stage", "Submission failure: {:#?}", e);
            return Err(e);
        };
        return match self
            .render_target
            .present_image(image_info, self.graphics_queue.clone())
        {
            Err(e) => {
                tagged_warn!("VkGraphics Stage", "Presentation failure: {:#?}", e);
                Err(e)
            }
            Ok(e) => Ok(e),
        };
    }

    fn window_render_target_binding(&self) -> &WindowRenderTargetBinding {
        &self.render_target
    }
    fn window_render_target_binding_mut(&mut self) -> &mut WindowRenderTargetBinding {
        &mut self.render_target
    }
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
