use std::ops::Deref;

use erupt::*;
use magnetar_engine::PlatformWindowHandle;

use crate::{
    config::VkGraphicsOptions, device::VkInitializedDevice, vk_device::VkDevice,
    vk_instance::VkInstance,
};
pub(crate) struct WindowRenderTargetBinding {
    // swapchain, etc.
    instance: VkInstance,
    device: VkDevice,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    window_handle: PlatformWindowHandle,
    surface: vk::SurfaceKHR,
}

impl Drop for WindowRenderTargetBinding {
    fn drop(&mut self) {
        unsafe {
            for image_view in &self.image_views {
                self.device.destroy_image_view(Some(*image_view), None);
            }
            self.images.clear();
            self.device
                .destroy_swapchain_khr(Some(self.swapchain), None);
            self.instance.destroy_surface_khr(Some(self.surface), None);
        }
    }
}

#[derive(Debug)]
pub enum WindowRenderTargetBindingError {
    NoSurfaceFormats,
    VkResultError(vk::Result),
}
impl From<vk::Result> for WindowRenderTargetBindingError {
    fn from(e: vk::Result) -> Self {
        Self::VkResultError(e)
    }
}

impl WindowRenderTargetBinding {
    pub fn new(
        instance: VkInstance,
        graphics_options: &VkGraphicsOptions,
        device: &VkInitializedDevice,
        window_handle: PlatformWindowHandle,
        surface: vk::SurfaceKHR,
    ) -> Result<Self, WindowRenderTargetBindingError> {
        let physical_dev = device.physical_device();

        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_dev, surface, None)
                .result()?
        };
        let surface_format = match formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR_KHR
            })
            .or_else(|| formats.get(0))
        {
            Some(f) => f,
            None => return Err(WindowRenderTargetBindingError::NoSurfaceFormats),
        };

        let present_mode = unsafe {
            instance.get_physical_device_surface_present_modes_khr(physical_dev, surface, None)
        }
        .result()?
        .into_iter()
        .find(|present_mode| {
            if graphics_options.prevent_tearing && !graphics_options.limit_frame_rate {
                present_mode == &vk::PresentModeKHR::MAILBOX_KHR
            } else if graphics_options.prevent_tearing && graphics_options.limit_frame_rate {
                present_mode == &vk::PresentModeKHR::FIFO_KHR
            } else {
                present_mode == &vk::PresentModeKHR::IMMEDIATE_KHR
            }
        })
        .unwrap_or(vk::PresentModeKHR::FIFO_KHR);

        let surface_caps = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_dev, surface)
                .result()?
        };
        let mut image_count = surface_caps.min_image_count + 1;
        if surface_caps.max_image_count > 0 && image_count > surface_caps.max_image_count {
            image_count = surface_caps.max_image_count;
        }

        let swapchain_info = vk::SwapchainCreateInfoKHRBuilder::new()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(surface_caps.current_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_caps.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain = unsafe { device.create_swapchain_khr(&swapchain_info, None) }.result()?;
        let swapchain_images =
            unsafe { device.get_swapchain_images_khr(swapchain, None) }.result()?;

        return Ok(Self {
            instance: instance,
            device: device.deref().clone(),
            swapchain: swapchain,
            images: swapchain_images,
            image_views: vec![],
            window_handle: window_handle,
            surface: surface,
        });
    }

    /// Get a reference to the window render target binding's surface.
    pub(crate) fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }

    /// Get a reference to the window render target binding's window handle.
    pub(crate) fn window_handle(&self) -> PlatformWindowHandle {
        self.window_handle
    }
}
