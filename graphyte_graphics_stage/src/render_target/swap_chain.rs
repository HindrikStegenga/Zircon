use crate::render_target::WindowRenderTarget;
use crate::{GraphicsDevice, GraphicsOptions};
use ash::extensions::khr::Surface;
use ash::vk::SwapchainKHR;
use ash::*;
use graphyte_engine::PlatformWindow;

pub(crate) struct SwapChain {
    swap_chain: vk::SwapchainKHR,
    loader: extensions::khr::Swapchain,
}

impl SwapChain {
    pub fn new(
        instance: &Instance,
        device: &GraphicsDevice,
        window: &dyn PlatformWindow,
        window_target: &WindowRenderTarget,
        options: &GraphicsOptions,
    ) -> Option<Self> {
        let surface = window_target.surface();
        let loader = window_target.loader();
        let surface_info = check_and_get_surface_info(surface, loader, device)?;
        let (loader, swap_chain) = create_swap_chain(
            instance,
            device.device(),
            window,
            surface,
            SwapchainKHR::null(),
            &surface_info,
            options,
        )?;

        Self { swap_chain, loader }.into()
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_swapchain(self.swap_chain, None);
        }
    }
}

struct SurfaceInfo {
    surface_caps: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

fn check_and_get_surface_info(
    surface: vk::SurfaceKHR,
    loader: &Surface,
    device: &GraphicsDevice,
) -> Option<SurfaceInfo> {
    let phys_device = device.physical_device();
    let qf_index = device.graphics_queue().qf_index;
    unsafe {
        // Check device surface support.
        if !loader
            .get_physical_device_surface_support(phys_device, qf_index, surface)
            .ok()?
        {
            return None;
        };
        SurfaceInfo {
            surface_caps: loader
                .get_physical_device_surface_capabilities(phys_device, surface)
                .ok()?,
            surface_formats: loader
                .get_physical_device_surface_formats(phys_device, surface)
                .ok()?,
            present_modes: loader
                .get_physical_device_surface_present_modes(phys_device, surface)
                .ok()?,
        }
        .into()
    }
}

fn select_extent(caps: &vk::SurfaceCapabilitiesKHR, window: &dyn PlatformWindow) -> vk::Extent2D {
    if caps.current_extent.width != u32::MAX || caps.current_extent.height != u32::MAX {
        return caps.current_extent;
    }
    let mut actual_extent = vk::Extent2D::builder()
        .width(window.width())
        .height(window.height())
        .build();
    actual_extent.width = u32::clamp(
        actual_extent.width,
        caps.min_image_extent.width,
        caps.max_image_extent.width,
    );
    actual_extent.height = u32::clamp(
        actual_extent.height,
        caps.min_image_extent.height,
        caps.max_image_extent.height,
    );
    actual_extent
}

fn select_image_count(caps: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let mut image_count = caps.min_image_count + 1;
    if caps.max_image_count > 0 && image_count > caps.max_image_count {
        image_count = caps.max_image_count;
    }
    image_count
}

fn select_present_mode(
    present_modes: &[vk::PresentModeKHR],
    options: &GraphicsOptions,
) -> vk::PresentModeKHR {
    *present_modes
        .iter()
        .find(|e| {
            if options.prevent_tearing && !options.limit_frame_rate {
                *e == &vk::PresentModeKHR::MAILBOX
            } else if options.prevent_tearing && options.limit_frame_rate {
                *e == &vk::PresentModeKHR::FIFO
            } else {
                *e == &vk::PresentModeKHR::IMMEDIATE
            }
        })
        .unwrap_or(&vk::PresentModeKHR::FIFO)
}

fn select_surface_format() -> vk::SurfaceFormatKHR {
    vk::SurfaceFormatKHR::builder()
        .format(vk::Format::B8G8R8A8_SRGB)
        .color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .build()
}

fn create_swap_chain(
    instance: &Instance,
    device: &Device,
    window: &dyn PlatformWindow,
    surface: vk::SurfaceKHR,
    old_swap_chain: vk::SwapchainKHR,
    surface_info: &SurfaceInfo,
    options: &GraphicsOptions,
) -> Option<(ash::extensions::khr::Swapchain, vk::SwapchainKHR)> {
    let surface_format = select_surface_format();
    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(select_image_count(&surface_info.surface_caps))
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(select_extent(&surface_info.surface_caps, window))
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(surface_info.surface_caps.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(select_present_mode(&surface_info.present_modes, options))
        .clipped(true)
        .old_swapchain(old_swap_chain);
    unsafe {
        let loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swap_chain = loader.create_swapchain(&create_info, None).ok()?;
        (loader, swap_chain).into()
    }
}
