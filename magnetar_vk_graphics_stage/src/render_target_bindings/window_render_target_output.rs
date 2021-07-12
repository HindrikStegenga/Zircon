use std::ops::Deref;

use erupt::*;
use magnetar_engine::{tagged_success, PlatformWindowHandle};

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
            Self::destroy_image_views(&self.device, &mut self.image_views);
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
        let physical_device = device.physical_device();

        let (swapchain, surface_format) = Self::create_swapchain(
            &instance,
            device,
            physical_device,
            surface,
            graphics_options,
            None,
        )?;

        tagged_success!("VkGraphics Stage", "Succesfully built Swapchain.");

        let (images, views) = Self::create_images_and_views(device, swapchain, surface_format)?;

        return Ok(Self {
            instance: instance,
            device: device.deref().clone(),
            swapchain: swapchain,
            images: images,
            image_views: views,
            window_handle: window_handle,
            surface: surface,
        });
    }

    fn destroy_image_views(device: &VkDevice, image_views: &mut Vec<vk::ImageView>) {
        image_views
            .iter()
            .for_each(|image_view| unsafe { device.destroy_image_view(Some(*image_view), None) });
        image_views.clear();
    }

    fn create_swapchain(
        instance: &VkInstance,
        device: &VkDevice,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        graphics_options: &VkGraphicsOptions,
        old_swap_chain: Option<vk::SwapchainKHR>,
    ) -> Result<(vk::SwapchainKHR, vk::SurfaceFormatKHR), WindowRenderTargetBindingError> {
        let (surface_caps, image_count) =
            Self::get_surface_capibilities_and_image_count(&instance, physical_device, surface)?;

        let present_mode =
            Self::select_present_mode(&instance, physical_device, surface, graphics_options)?;
        let surface_format = Self::select_surface_format(&instance, physical_device, surface)?;

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
            .old_swapchain(old_swap_chain.unwrap_or(vk::SwapchainKHR::null()));

        Ok((
            unsafe { device.create_swapchain_khr(&swapchain_info, None) }.result()?,
            surface_format,
        ))
    }

    fn create_images_and_views(
        device: &VkDevice,
        swapchain: vk::SwapchainKHR,
        surface_format: vk::SurfaceFormatKHR,
    ) -> Result<(Vec<vk::Image>, Vec<vk::ImageView>), vk::Result> {
        let swapchain_images =
            unsafe { device.get_swapchain_images_khr(swapchain, None) }.result()?;

        let mut swapchain_image_views: Vec<_> = Vec::with_capacity(swapchain_images.len());
        for swapchain_image in &swapchain_images {
            let image_view_info = vk::ImageViewCreateInfoBuilder::new()
                .image(*swapchain_image)
                .view_type(vk::ImageViewType::_2D)
                .format(surface_format.format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(
                    vk::ImageSubresourceRangeBuilder::new()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                );
            let image_view =
                unsafe { device.create_image_view(&image_view_info, None) }.result()?;
            swapchain_image_views.push(image_view);
        }
        Ok((swapchain_images, swapchain_image_views))
    }

    fn get_surface_capibilities_and_image_count(
        instance: &VkInstance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<(vk::SurfaceCapabilitiesKHR, u32), vk::Result> {
        let surface_caps = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_device, surface)
                .result()?
        };
        let mut image_count = surface_caps.min_image_count + 1;
        if surface_caps.max_image_count > 0 && image_count > surface_caps.max_image_count {
            image_count = surface_caps.max_image_count;
        }
        Ok((surface_caps, image_count))
    }

    fn select_surface_format(
        instance: &VkInstance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::SurfaceFormatKHR, WindowRenderTargetBindingError> {
        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_device, surface, None)
                .result()?
        };

        match formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR_KHR
            })
            .or_else(|| formats.get(0))
        {
            Some(f) => Ok(*f),
            None => Err(WindowRenderTargetBindingError::NoSurfaceFormats),
        }
    }

    fn select_present_mode(
        instance: &VkInstance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        graphics_options: &VkGraphicsOptions,
    ) -> Result<vk::PresentModeKHR, vk::Result> {
        Ok(unsafe {
            instance.get_physical_device_surface_present_modes_khr(physical_device, surface, None)
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
        .unwrap_or(vk::PresentModeKHR::FIFO_KHR))
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
