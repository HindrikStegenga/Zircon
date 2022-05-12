use std::sync::Arc;
use crate::render_target::WindowRenderTarget;
use crate::{GraphicsDevice, GraphicsOptions};
use ash::extensions::khr::Surface;
use ash::vk::{ImageView, SwapchainKHR};
use ash::*;
use graphyte_engine::PlatformWindow;

pub(crate) struct FrameSyncData {
    image_available_semaphore: vk::Semaphore,
    render_finished_semaphore: vk::Semaphore,
    render_finished_fence: vk::Fence,
}

impl FrameSyncData {
    pub fn image_available_semaphore(&self) -> vk::Semaphore {
        self.image_available_semaphore
    }
    pub fn render_finished_semaphore(&self) -> vk::Semaphore {
        self.render_finished_semaphore
    }
    pub fn render_finished_fence(&self) -> vk::Fence {
        self.render_finished_fence
    }
}

pub(crate) struct SwapChain {
    image_views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    swap_chain: vk::SwapchainKHR,
    loader: extensions::khr::Swapchain,
    device: Arc<Device>
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
        let surface_format = select_surface_format();
        let (loader, swap_chain) = create_swap_chain(
            instance,
            device.device(),
            window,
            surface,
            surface_format,
            SwapchainKHR::null(),
            &surface_info,
            options,
        )?;
        let (images, image_views) = create_images_and_views(device.device(), &loader, swap_chain, surface_format).ok()?;
        Self { image_views, images, swap_chain, loader, device: device.device_arc() }.into()
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            destroy_image_views(&self.device, &mut self.image_views);
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
    surface_format: vk::SurfaceFormatKHR,
    old_swap_chain: vk::SwapchainKHR,
    surface_info: &SurfaceInfo,
    options: &GraphicsOptions,
) -> Option<(ash::extensions::khr::Swapchain, vk::SwapchainKHR)> {
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

fn create_images_and_views(
    device: &Device,
    sw_loader: &ash::extensions::khr::Swapchain,
    swap_chain: vk::SwapchainKHR,
    surface_format: vk::SurfaceFormatKHR,
) -> Result<(Vec<vk::Image>, Vec<vk::ImageView>), vk::Result> {
    let images = unsafe { sw_loader.get_swapchain_images(swap_chain) }?;

    let mut image_views : Vec<vk::ImageView> = vec![];
    for swap_chain_image in &images {
        let image_view_info = vk::ImageViewCreateInfo::builder()
            .image(*swap_chain_image)
            .format(surface_format.format)
            .view_type(vk::ImageViewType::TYPE_2D)
            .components(
                vk::ComponentMapping::builder()
                    .r(vk::ComponentSwizzle::IDENTITY)
                    .g(vk::ComponentSwizzle::IDENTITY)
                    .b(vk::ComponentSwizzle::IDENTITY)
                    .a(vk::ComponentSwizzle::IDENTITY)
                    .build())
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build()
            );
        let image_view = unsafe { device.create_image_view(&image_view_info, None)? };
        image_views.push(image_view);
    }

    Ok((images, image_views))
}

fn destroy_image_views(device: &Device, image_views: &mut Vec<vk::ImageView>) {
    image_views.iter().for_each(|img_view| unsafe {
        device.destroy_image_view(*img_view, None);
    });
    image_views.clear();
}

fn create_synchronization_primitives(device: &Device, frames_in_flight: u32) -> Result<Vec<FrameSyncData>, vk::Result> {
    let frames_in_flight = std::cmp::max(frames_in_flight, 1);
    let mut frame_sync_data = vec![];

    for _ in 0..frames_in_flight {
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder().build();
        let semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None)? };

    }

    Ok(frame_sync_data)
}