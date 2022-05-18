use crate::{
    render_target::{SwapChain, WindowRenderTarget},
    Camera, GraphicsDevice, RenderPath, RenderPathCreateInfo,
};

use ash::*;
use graphyte_engine::*;
use std::{ffi::CString, sync::Arc};

struct FrameCommandBuffer {
    command_pool: vk::CommandPool,
    main_command_buffer: vk::CommandBuffer,
}

pub struct ForwardRenderPath {
    device: Arc<Device>,
    frame_command_buffers: Vec<FrameCommandBuffer>,
    frame_buffers: Vec<vk::Framebuffer>,
    render_pass: vk::RenderPass,
}

impl Drop for ForwardRenderPath {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().ok();

            for fb in &self.frame_buffers {
                self.device.destroy_framebuffer(*fb, None);
            }

            self.device.destroy_render_pass(self.render_pass, None);

            for fcb in &self.frame_command_buffers {
                self.device.destroy_command_pool(fcb.command_pool, None);
            }
            self.frame_command_buffers.clear();
        }
    }
}

impl ForwardRenderPath {
    fn init_default_command_pool(device: &GraphicsDevice) -> Result<vk::CommandPool, vk::Result> {
        let queue = device.graphics_queue();

        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue.qf_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        unsafe {
            device
                .device()
                .create_command_pool(&command_pool_info, None)
        }
    }

    fn init_default_command_buffer(
        device: &Device,
        command_pool: vk::CommandPool,
    ) -> Result<vk::CommandBuffer, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(vk::CommandBufferLevel::PRIMARY);

        unsafe {
            let bufs = device.allocate_command_buffers(&alloc_info)?;
            return Ok(*bufs.first().unwrap());
        }
    }

    fn init_default_render_pass(
        device: &Device,
        swap_image_format: vk::Format,
    ) -> Result<vk::RenderPass, vk::Result> {
        // Describe the color attachment
        let color_attachment = [vk::AttachmentDescription::builder()
            .format(swap_image_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build()];

        // Create a reference object which refers to the color attachment
        let color_attachment_reference = [vk::AttachmentReference::builder()
            .attachment(0) // color is at 0
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()]; // We start first subpass with color optimal layout

        // Define the first and only default subpass.
        let subpass = [vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachment_reference)
            .build()]; // Connect up the color attachment to the subpass.

        // Set up the render pass info, inserting the color attachment and the subpasses it needs.
        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&color_attachment)
            .subpasses(&subpass);

        unsafe { device.create_render_pass(&render_pass_info, None) }
    }

    fn init_default_frame_buffers(
        device: &Device,
        render_pass: vk::RenderPass,
        swap_chain: &SwapChain,
    ) -> Result<Vec<vk::Framebuffer>, vk::Result> {
        let mut frame_buffers = Vec::with_capacity(swap_chain.image_count() as usize);
        for i in 0..swap_chain.image_count() {
            let image_view = swap_chain.image_view(i as usize);
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(core::slice::from_ref(&image_view))
                .width(swap_chain.current_extent().width)
                .height(swap_chain.current_extent().height)
                .layers(1);
            let buffer = unsafe { device.create_framebuffer(&create_info, None) }?;
            frame_buffers.push(buffer);
        }

        Ok(frame_buffers)
    }
}

impl RenderPath for ForwardRenderPath {
    fn render_path_identifier() -> CString
    where
        Self: Sized,
    {
        CString::new(b"Forward" as &[u8]).unwrap()
    }

    fn required_device_extensions() -> Vec<CString>
    where
        Self: Sized,
    {
        vec![CString::from(ash::extensions::khr::Swapchain::name())]
    }

    fn required_device_features() -> vk::PhysicalDeviceFeatures
    where
        Self: Sized,
    {
        vk::PhysicalDeviceFeatures::default()
    }

    fn instantiate(create_info: RenderPathCreateInfo) -> Option<Self>
    where
        Self: Sized,
    {
        let render_pass = Self::init_default_render_pass(
            create_info.graphics_device.device(),
            create_info.swap_chain.surface_format().format,
        )
        .ok()?;

        let mut frame_command_buffers =
            Vec::with_capacity(create_info.swap_chain.image_count() as usize);

        for _ in 0..create_info.swap_chain.image_count() {
            let command_pool = Self::init_default_command_pool(create_info.graphics_device).ok()?;
            // TODO: Handle proper destruction in case a cmd pool fails to be created.
            let command_buffer = Self::init_default_command_buffer(
                create_info.graphics_device.device(),
                command_pool,
            )
            .ok()?;
            frame_command_buffers.push(FrameCommandBuffer {
                command_pool,
                main_command_buffer: command_buffer,
            })
        }

        let frame_buffers = Self::init_default_frame_buffers(
            create_info.graphics_device.device(),
            render_pass,
            create_info.swap_chain,
        )
        .ok()?;

        Self {
            frame_command_buffers,
            device: Arc::clone(&create_info.graphics_device.device_arc()),
            render_pass,
            frame_buffers,
        }
        .into()
    }

    fn render(
        &mut self,
        _camera: &Camera,
        swap_chain: &mut SwapChain,
        _window_render_target: &mut WindowRenderTarget,
        device: &GraphicsDevice,
    ) -> bool {
        unsafe {
            let (info, is_sub_optimal) = match swap_chain.acquire_next_frame() {
                Ok(value) => value,
                Err(e) => match e {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        // TODO: Handle swap resize/recreate!
                        return true;
                    }
                    _ => {
                        return false;
                    }
                },
            };

            // Start drawing

            let command_frame_buf = &self.frame_command_buffers[info.image_index as usize];

            device
                .device()
                .reset_command_pool(
                    command_frame_buf.command_pool,
                    vk::CommandPoolResetFlags::empty(),
                )
                .ok();

            let main_command_buffer = command_frame_buf.main_command_buffer;

            let cmd_begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
                .build();

            // Record commands
            device
                .device()
                .begin_command_buffer(main_command_buffer, &cmd_begin_info)
                .ok();

            let clear_value = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [1.0, 1.0, 1.0, 1.0],
                },
            }];

            let rp_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: swap_chain.current_extent(),
                })
                .framebuffer(self.frame_buffers[info.image_index as usize])
                .clear_values(&clear_value);

            device.device().cmd_begin_render_pass(
                main_command_buffer,
                &rp_begin_info,
                vk::SubpassContents::INLINE,
            );

            device.device().cmd_end_render_pass(main_command_buffer);

            device.device().end_command_buffer(main_command_buffer).ok();

            // Submit our command buffer to the queue

            let submit_info = vk::SubmitInfo::builder()
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .wait_semaphores(&[info.wait_semaphore])
                .signal_semaphores(&[info.rendering_finished_semaphore])
                .command_buffers(&[main_command_buffer])
                .build();

            device
                .device()
                .queue_submit(
                    device.graphics_queue().queue,
                    &[submit_info],
                    info.rendering_finished_fence,
                )
                .ok();

            // Present the frame to the screen

            match swap_chain.present_frame(info.image_index, &device.graphics_queue()) {
                Ok(is_sub_optimal) => {
                    tagged_log!("Graphics", "Surface became sub-optimal.");
                }
                Err(e) => {
                    // TODO: Handle swap resize/recreate!
                    tagged_error!("Graphics", "Presentation error: {}", e);
                    return false;
                }
            }
        }
        return true;
    }
}
