use crate::render_target::WindowRenderTarget;
use crate::{DeviceQueue, GraphicsDevice, GraphicsOptions};
use ash::extensions::khr::Surface;
use ash::vk::Extent2D;
use ash::*;
use engine::PlatformWindow;
use std::sync::Arc;
use utils::*;

// Wraps the vulkan swap chain and it's associated images and imageviews.
pub struct SwapChain {
    current_frame_index: u32,
    current_extent: vk::Extent2D,
    frames_in_flight: u32,
    image_available_semaphores: Vec<vk::Semaphore>,
    rendering_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    image_views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    surface_format: vk::SurfaceFormatKHR,
    swap_chain: vk::SwapchainKHR,
    swapchain_loader: extensions::khr::Swapchain,
    device: Arc<Device>,
}

pub struct AcquiredFrameInfo {
    // Image index that needs to be passed to [`present_frame`].
    pub image_index: u32,
    // Semaphore that needs to be waited on before any rendering may occur.
    pub wait_semaphore: vk::Semaphore,
    // Semaphore that needs to be signaled when rendering is finished.
    pub rendering_finished_semaphore: vk::Semaphore,
    // Fence that needs to be signaled when rendering is finished.
    // This is to synchronize the CPU and GPU.
    pub rendering_finished_fence: vk::Fence,
    // Framebuffer size
    pub current_extent: Extent2D,
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
        let surface_loader = window_target.loader();
        let surface_info = check_and_get_surface_info(surface, surface_loader, device).ok()?;

        let surface_format = select_surface_format(&surface_info);
        let swap_loader = ash::extensions::khr::Swapchain::new(instance, device.device());

        let (swap_chain, extent) = match create_swap_chain(
            device.device(),
            &swap_loader,
            window,
            surface,
            surface_format,
            vk::SwapchainKHR::null(),
            &surface_info,
            options,
        ) {
            Ok(v) => v,
            Err(e) => {
                t_error!("Error occurred during swap chain creation: {}", e);
                return None;
            }
        };
        let (images, image_views) =
            create_images_and_views(device.device(), &swap_loader, swap_chain, surface_format)
                .ok()?;
        let (image_available_semaphores, rendering_finished_semaphores, in_flight_fences) =
            create_synchronization_primitives(device.device(), options.preferred_frames_in_flight)
                .ok()?;

        Self {
            image_views,
            images,
            swap_chain,
            swapchain_loader: swap_loader,
            device: device.device_arc(),
            image_available_semaphores,
            rendering_finished_semaphores,
            in_flight_fences,
            current_frame_index: 0,
            frames_in_flight: options.preferred_frames_in_flight,
            surface_format,
            current_extent: extent,
        }
        .into()
    }

    pub fn current_extent(&self) -> vk::Extent2D {
        self.current_extent
    }

    pub fn image_view(&self, image_index: usize) -> vk::ImageView {
        self.image_views[image_index]
    }

    pub fn resize_swap_chain(
        &mut self,
        window: &dyn PlatformWindow,
        window_target: &WindowRenderTarget,
        _width: u32,
        _height: u32,
        options: &GraphicsOptions,
        device: &GraphicsDevice,
    ) -> Result<(), vk::Result> {
        let surface_loader = window_target.loader();
        let surface_info =
            check_and_get_surface_info(window_target.surface(), surface_loader, device)?;

        for fb in &self.image_views {
            unsafe {
                self.device.destroy_image_view(*fb, None);
            }
        }
        self.image_views.clear();
        destroy_syncronization_primitives(
            &self.device,
            &mut self.image_available_semaphores,
            &mut self.rendering_finished_semaphores,
            &mut self.in_flight_fences,
        );

        self.current_frame_index = 0;
        let (new_swap, new_extent) = create_swap_chain(
            &self.device,
            &self.swapchain_loader,
            window,
            window_target.surface(),
            self.surface_format(),
            self.swap_chain,
            &surface_info,
            options,
        )?;
        let old_swap = self.swap_chain;
        self.swap_chain = new_swap;
        self.current_extent = new_extent;

        let (images, image_views) = create_images_and_views(
            &self.device,
            &self.swapchain_loader,
            self.swap_chain,
            self.surface_format,
        )?;
        self.images = images;
        self.image_views = image_views;

        let (image_available_semaphores, rendering_finished_semaphores, in_flight_fences) =
            create_synchronization_primitives(&self.device, options.preferred_frames_in_flight)?;

        self.image_available_semaphores = image_available_semaphores;
        self.rendering_finished_semaphores = rendering_finished_semaphores;
        self.in_flight_fences = in_flight_fences;

        unsafe {
            self.swapchain_loader.destroy_swapchain(old_swap, None);
        }
        Ok(())
    }

    // Each call of this function MUST be matched with a call to [`present_frame`].
    // Unless a failure happens and the swap chain is recreated in which case it's not necessary.
    pub unsafe fn acquire_next_frame(&mut self) -> Result<(AcquiredFrameInfo, bool), vk::Result> {
        let device = self.device.as_ref();
        const DEFAULT_TIME_OUT: u64 = u64::MAX;

        // Wait for fence
        match device.wait_for_fences(
            core::slice::from_ref(&self.in_flight_fences[self.current_frame_index as usize]),
            true,
            DEFAULT_TIME_OUT,
        ) {
            Ok(_) => (),
            Err(e) => {
                t_error!("Wait for fence error! {}", e);
                return Err(e);
            }
        };

        // Acquire an image
        let (image_index, suboptimal) = match self.swapchain_loader.acquire_next_image(
            self.swap_chain,
            DEFAULT_TIME_OUT,
            self.image_available_semaphores[self.current_frame_index as usize],
            vk::Fence::null(),
        ) {
            Ok(v) => v,
            Err(e) => {
                t_error!("Acquire error! {}", e);
                return Err(e);
            }
        };

        // Reset the fence
        match device.reset_fences(core::slice::from_ref(
            &self.in_flight_fences[self.current_frame_index as usize],
        )) {
            Ok(_) => (),
            Err(e) => {
                t_error!("Fence reset error! {}", e);
                return Err(e);
            }
        };

        Ok((
            AcquiredFrameInfo {
                wait_semaphore: self.image_available_semaphores[self.current_frame_index as usize],
                rendering_finished_semaphore: self.rendering_finished_semaphores
                    [self.current_frame_index as usize],
                rendering_finished_fence: self.in_flight_fences[self.current_frame_index as usize],
                image_index,
                current_extent: self.current_extent(),
            },
            suboptimal,
        ))
    }

    // Needs to be called in tandem with [`acquire_next_frame`].
    pub unsafe fn present_frame(
        &mut self,
        image_index: u32,
        present_queue: &DeviceQueue,
    ) -> Result<bool, vk::Result> {
        let current_frame_index = self.current_frame_index as usize;

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(core::slice::from_ref(
                &self.rendering_finished_semaphores[current_frame_index],
            ))
            .swapchains(core::slice::from_ref(&self.swap_chain))
            .image_indices(core::slice::from_ref(&image_index));

        let value = match {
            self.swapchain_loader
                .queue_present(present_queue.queue, &present_info)
        } {
            Ok(is_sub_optimal) => Ok(is_sub_optimal),
            Err(error) => Err(error),
        };

        self.current_frame_index = (self.current_frame_index + 1u32) % self.frames_in_flight;
        return value;
    }

    pub fn image_count(&self) -> u32 {
        self.images.len() as u32
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        self.surface_format
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            destroy_syncronization_primitives(
                &self.device,
                &mut self.image_available_semaphores,
                &mut self.rendering_finished_semaphores,
                &mut self.in_flight_fences,
            );
            destroy_image_views(&self.device, &mut self.image_views);
            self.swapchain_loader
                .destroy_swapchain(self.swap_chain, None);
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
) -> Result<SurfaceInfo, vk::Result> {
    let phys_device = device.physical_device();
    let qf_index = device.graphics_queue().qf_index;
    unsafe {
        // Check device surface support.
        match loader.get_physical_device_surface_support(phys_device, qf_index, surface) {
            Ok(v) => {
                if !v {
                    t_error!("Surface not supported");
                    return Err(vk::Result::ERROR_FORMAT_NOT_SUPPORTED);
                }
            }
            Err(e) => return Err(e),
        };

        Ok(SurfaceInfo {
            surface_caps: loader.get_physical_device_surface_capabilities(phys_device, surface)?,
            surface_formats: loader.get_physical_device_surface_formats(phys_device, surface)?,
            present_modes: loader
                .get_physical_device_surface_present_modes(phys_device, surface)?,
        })
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

fn select_surface_format(_surface_info: &SurfaceInfo) -> vk::SurfaceFormatKHR {
    // TODO: Check actual supported surfaces!
    vk::SurfaceFormatKHR::builder()
        .format(vk::Format::B8G8R8A8_SRGB)
        .color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
        .build()
}

fn create_swap_chain(
    device: &Device,
    swap_loader: &ash::extensions::khr::Swapchain,
    window: &dyn PlatformWindow,
    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    old_swap_chain: vk::SwapchainKHR,
    surface_info: &SurfaceInfo,
    options: &GraphicsOptions,
) -> Result<(vk::SwapchainKHR, vk::Extent2D), vk::Result> {
    let image_count = select_image_count(&surface_info.surface_caps);
    let extent = select_extent(&surface_info.surface_caps, window);
    let present_mode = select_present_mode(&surface_info.present_modes, options);

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(surface_info.surface_caps.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swap_chain);
    unsafe {
        let swap_chain = swap_loader.create_swapchain(&create_info, None)?;
        Ok((swap_chain, extent))
    }
}

fn create_images_and_views(
    device: &Device,
    sw_loader: &ash::extensions::khr::Swapchain,
    swap_chain: vk::SwapchainKHR,
    surface_format: vk::SurfaceFormatKHR,
) -> Result<(Vec<vk::Image>, Vec<vk::ImageView>), vk::Result> {
    let images = unsafe { sw_loader.get_swapchain_images(swap_chain) }?;

    let mut image_views: Vec<vk::ImageView> = vec![];
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
                    .build(),
            )
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
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

fn destroy_syncronization_primitives(
    device: &Device,
    image_available_semaphores: &mut Vec<vk::Semaphore>,
    rendering_finished_semaphores: &mut Vec<vk::Semaphore>,
    in_flight_fences: &mut Vec<vk::Fence>,
) {
    for semaphore in image_available_semaphores.iter() {
        unsafe {
            device.destroy_semaphore(*semaphore, None);
        }
    }
    for semaphore in rendering_finished_semaphores.iter() {
        unsafe {
            device.destroy_semaphore(*semaphore, None);
        }
    }
    for fence in in_flight_fences.iter() {
        unsafe {
            device.destroy_fence(*fence, None);
        }
    }
    image_available_semaphores.clear();
    rendering_finished_semaphores.clear();
    in_flight_fences.clear();
}

fn create_synchronization_primitives(
    device: &Device,
    frames_in_flight: u32,
) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>), vk::Result> {
    let frames_in_flight = std::cmp::max(frames_in_flight, 1);

    let mut image_available_semaphores: Vec<vk::Semaphore> =
        Vec::with_capacity(frames_in_flight as usize);
    let mut rendering_finished_semaphores: Vec<vk::Semaphore> =
        Vec::with_capacity(frames_in_flight as usize);
    let mut in_flight_fences: Vec<vk::Fence> = Vec::with_capacity(frames_in_flight as usize);

    for _ in 0..frames_in_flight {
        // Fill image available semaphore
        let semaphore_builder = vk::SemaphoreCreateInfo::builder();
        match { unsafe { device.create_semaphore(&semaphore_builder, None) } } {
            Ok(v) => image_available_semaphores.push(v),
            Err(e) => {
                destroy_syncronization_primitives(
                    device,
                    &mut image_available_semaphores,
                    &mut rendering_finished_semaphores,
                    &mut in_flight_fences,
                );
                return Err(e);
            }
        }
        // Fill render finished semaphore
        match { unsafe { device.create_semaphore(&semaphore_builder, None) } } {
            Ok(v) => rendering_finished_semaphores.push(v),
            Err(e) => {
                destroy_syncronization_primitives(
                    device,
                    &mut image_available_semaphores,
                    &mut rendering_finished_semaphores,
                    &mut in_flight_fences,
                );
                return Err(e);
            }
        }
        // Fill fences for command buffers
        let fence_builder = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
        match { unsafe { device.create_fence(&fence_builder, None) } } {
            Ok(v) => in_flight_fences.push(v),
            Err(e) => {
                destroy_syncronization_primitives(
                    device,
                    &mut image_available_semaphores,
                    &mut rendering_finished_semaphores,
                    &mut in_flight_fences,
                );
                return Err(e);
            }
        }
    }

    Ok((
        image_available_semaphores,
        rendering_finished_semaphores,
        in_flight_fences,
    ))
}
