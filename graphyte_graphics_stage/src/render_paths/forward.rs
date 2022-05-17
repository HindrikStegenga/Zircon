use crate::{
    render_target::{SwapChain, WindowRenderTarget},
    Camera, GraphicsDevice, GraphicsOptions, RenderPath, RenderPathCreateInfo,
};

use ash::*;
use graphyte_engine::*;
use std::ffi::CString;

pub struct ForwardRenderPath {}

impl ForwardRenderPath {
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
        Self {}.into()
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

            // Process drawing

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
