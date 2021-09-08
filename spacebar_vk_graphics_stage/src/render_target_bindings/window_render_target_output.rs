use std::ops::Deref;

use erupt::{vk::CommandBuffer, *};
use spacebar_engine::{
    tagged_log, tagged_success, PlatformInterface, PlatformWindow, PlatformWindowHandle,
};

use crate::device::fence::VkFence;
use crate::{
    config::VkGraphicsOptions,
    device::{VkInitializedDevice, VkQueue},
    vk_device::VkDevice,
    vk_instance::VkInstance,
};
use erupt::utils::VulkanResult;

pub(crate) struct WindowRenderTargetBinding {
    in_flight_fences: Vec<VkFence>,
    images_in_flight: Vec<vk::Fence>,
    current_frame_index: u32,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    graphics_options: VkGraphicsOptions,
    physical_device: vk::PhysicalDevice,
    window_handle: PlatformWindowHandle,
    image_views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    surface_extent: vk::Extent2D,
    surface_format: vk::SurfaceFormatKHR,
    swapchain: vk::SwapchainKHR,
    surface: vk::SurfaceKHR,
    device: VkDevice,
    instance: VkInstance,
}

impl Drop for WindowRenderTargetBinding {
    fn drop(&mut self) {
        unsafe {
            self.images_in_flight.clear();
            self.in_flight_fences.clear();
            Self::destroy_semaphores(&self.device, &mut self.image_available_semaphores);
            Self::destroy_semaphores(&self.device, &mut self.render_finished_semaphores);
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
    InvalidWindowHandle,
    VkResultError(vk::Result),
}
impl From<vk::Result> for WindowRenderTargetBindingError {
    fn from(e: vk::Result) -> Self {
        Self::VkResultError(e)
    }
}

pub struct AcquiredImageInfo {
    pub image_index: u32,
    pub cmd_submission_fence: vk::Fence,
    pub render_finished_semaphore: vk::Semaphore,
    pub image_available_semaphore: vk::Semaphore,
}

pub enum PresentImageInfo {
    Acquired(AcquiredImageInfo),
    SubOptimal(AcquiredImageInfo),
    OutOfDate,
}

pub enum PresentResult {
    Success,
    SubOptimal,
    OutOfDate,
}

impl WindowRenderTargetBinding {
    pub fn acquire_next_image(&mut self) -> Result<PresentImageInfo, vk::Result> {
        let fence = &self.in_flight_fences[self.current_frame_index as usize];
        fence.wait()?;

        let acq_result = unsafe {
            self.device.acquire_next_image_khr(
                self.swapchain,
                u64::MAX,
                Some(self.image_available_semaphores[self.current_frame_index as usize]),
                None,
            )
        };

        match acq_result {
            VulkanResult {
                value: Some(image_index),
                raw: vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR,
            } => {
                // Check if a previous frame is using this image.
                let possible_fence = [self.images_in_flight[image_index as usize]];
                if possible_fence[0] != vk::Fence::null() {
                    unsafe {
                        self.device
                            .wait_for_fences(&possible_fence, true, u64::MAX)
                            .result()?
                    };
                }
                self.images_in_flight[image_index as usize] =
                    self.in_flight_fences[self.current_frame_index as usize].handle();

                Ok(PresentImageInfo::Acquired(AcquiredImageInfo {
                    image_index,
                    cmd_submission_fence: fence.handle(),
                    render_finished_semaphore: self.render_finished_semaphores
                        [self.current_frame_index as usize],
                    image_available_semaphore: self.image_available_semaphores
                        [self.current_frame_index as usize],
                }))
            }
            VulkanResult {
                value: _,
                raw: vk::Result::ERROR_OUT_OF_DATE_KHR,
            } => Ok(PresentImageInfo::OutOfDate),
            VulkanResult { value: _, raw } => return Err(raw),
        }
    }

    pub fn present_image(
        &mut self,
        image_info: AcquiredImageInfo,
        queue: VkQueue,
    ) -> Result<PresentResult, vk::Result> {
        let render_finished = &self.render_finished_semaphores
            [self.current_frame_index as usize..self.current_frame_index as usize + 1];
        let image_indices = [image_info.image_index];
        let swapchain = [self.swapchain];
        let present_info = vk::PresentInfoKHRBuilder::new()
            .wait_semaphores(&render_finished)
            .image_indices(&image_indices)
            .swapchains(&swapchain);

        // Determine semaphore to use and fence to wait on for next frame.
        self.current_frame_index =
            (self.current_frame_index + 1) % self.in_flight_fences.len() as u32;

        let result = unsafe { self.device.queue_present_khr(queue.queue, &present_info) };
        return match result {
            VulkanResult {
                value: _,
                raw: vk::Result::SUCCESS,
            } => Ok(PresentResult::Success),
            VulkanResult {
                value: _,
                raw: vk::Result::SUBOPTIMAL_KHR,
            } => Ok(PresentResult::SubOptimal),
            VulkanResult {
                value: _,
                raw: vk::Result::ERROR_OUT_OF_DATE_KHR,
            } => Ok(PresentResult::OutOfDate),
            VulkanResult { value: _, raw } => Err(raw),
        };
    }

    /// Creates a new window render target binding. Creates swapchain, images and sync objects. Takes ownership of the provided surface.
    pub fn new(
        instance: VkInstance,
        graphics_options: &VkGraphicsOptions,
        device: &VkInitializedDevice,
        platform_interface: &dyn PlatformInterface,
        window_handle: PlatformWindowHandle,
        surface: vk::SurfaceKHR,
    ) -> Result<Self, WindowRenderTargetBindingError> {
        let physical_device = device.physical_device();
        let window = match platform_interface.get_window(window_handle) {
            Some(v) => v,
            None => return Err(WindowRenderTargetBindingError::InvalidWindowHandle),
        };

        let (caps, swapchain, surface_format) = Self::create_swapchain(
            &instance,
            device,
            physical_device,
            surface,
            graphics_options,
            None,
            window,
        )?;

        let frames_in_flight = std::cmp::max(graphics_options.preferred_frames_in_flight, 1);

        tagged_success!("VkGraphics Stage", "Succesfully built Swapchain.");

        let (mut images, mut image_views) =
            Self::create_images_and_views(device, swapchain, surface_format).map_err(
                |e| unsafe {
                    device.destroy_swapchain_khr(Some(swapchain), None);
                    instance.destroy_surface_khr(Some(surface), None);
                    e
                },
            )?;

        let (mut image_available_semaphores, mut render_finished_semaphores) =
            Self::create_semaphores(device, frames_in_flight).map_err(|e| unsafe {
                Self::destroy_image_views(device, &mut image_views);
                images.clear();
                device.destroy_swapchain_khr(Some(swapchain), None);
                instance.destroy_surface_khr(Some(surface), None);
                e
            })?;

        let in_flight_fences =
            Self::create_fences(device, frames_in_flight).map_err(|e| unsafe {
                Self::destroy_semaphores(device, &mut image_available_semaphores);
                Self::destroy_semaphores(device, &mut render_finished_semaphores);
                Self::destroy_image_views(device, &mut image_views);
                images.clear();
                device.destroy_swapchain_khr(Some(swapchain), None);
                instance.destroy_surface_khr(Some(surface), None);
                e
            })?;

        return Ok(Self {
            images_in_flight: (0..images.len()).map(|_| vk::Fence::null()).collect(),
            current_frame_index: 0,
            instance,
            device: device.deref().clone(),
            swapchain,
            images,
            image_views,
            window_handle,
            surface,
            surface_format,
            surface_extent: caps.current_extent,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            graphics_options: graphics_options.clone(),
            physical_device: physical_device,
        });
    }

    fn destroy_image_views(device: &VkDevice, image_views: &mut Vec<vk::ImageView>) {
        image_views
            .iter()
            .for_each(|image_view| unsafe { device.destroy_image_view(Some(*image_view), None) });
        image_views.clear();
    }

    fn destroy_semaphores(device: &VkDevice, semaphores: &mut Vec<vk::Semaphore>) {
        semaphores
            .iter()
            .for_each(|s| unsafe { device.destroy_semaphore(Some(*s), None) });
        semaphores.clear();
    }

    fn create_swapchain(
        instance: &VkInstance,
        device: &VkDevice,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        graphics_options: &VkGraphicsOptions,
        old_swap_chain: Option<vk::SwapchainKHR>,
        window: &dyn PlatformWindow,
    ) -> Result<
        (
            vk::SurfaceCapabilitiesKHR,
            vk::SwapchainKHR,
            vk::SurfaceFormatKHR,
        ),
        WindowRenderTargetBindingError,
    > {
        let (surface_caps, image_count) =
            Self::select_surface_caps_image_count(&instance, physical_device, surface)?;

        let extent = Self::select_extent(&surface_caps, window);

        let present_mode =
            Self::select_present_mode(&instance, physical_device, surface, graphics_options)?;
        let surface_format = Self::select_surface_format(&instance, physical_device, surface)?;

        let swapchain_info = vk::SwapchainCreateInfoKHRBuilder::new()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_caps.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(old_swap_chain.unwrap_or(vk::SwapchainKHR::null()));

        Ok((
            surface_caps,
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

    fn select_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window: &dyn PlatformWindow,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            return capabilities.current_extent;
        }

        let mut actual_extent = vk::Extent2DBuilder::new()
            .width(window.width())
            .height(window.height())
            .build();

        actual_extent.width = u32::clamp(
            actual_extent.width,
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        );
        actual_extent.height = u32::clamp(
            actual_extent.height,
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        );

        return actual_extent;
    }

    fn select_surface_caps_image_count(
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

    fn create_semaphores(
        device: &VkDevice,
        frames_in_flight: u32,
    ) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>), vk::Result> {
        let frames_in_flight = std::cmp::max(frames_in_flight, 1);
        let create_info = vk::SemaphoreCreateInfoBuilder::new();
        let mut image_available_semaphores: Vec<vk::Semaphore> =
            Vec::with_capacity(frames_in_flight as usize);
        let mut render_finished_semaphores: Vec<vk::Semaphore> =
            Vec::with_capacity(frames_in_flight as usize);

        for _ in 0..frames_in_flight {
            let semaphore = match unsafe { device.create_semaphore(&create_info, None).result() } {
                Ok(v) => v,
                Err(e) => {
                    Self::destroy_semaphores(device, &mut image_available_semaphores);
                    return Err(e);
                }
            };
            image_available_semaphores.push(semaphore);
        }
        for _ in 0..frames_in_flight {
            let semaphore = match unsafe { device.create_semaphore(&create_info, None).result() } {
                Ok(v) => v,
                Err(e) => {
                    Self::destroy_semaphores(device, &mut image_available_semaphores);
                    Self::destroy_semaphores(device, &mut render_finished_semaphores);
                    return Err(e);
                }
            };
            render_finished_semaphores.push(semaphore);
        }

        Ok((image_available_semaphores, render_finished_semaphores))
    }

    fn create_fences(device: &VkDevice, frames_in_flight: u32) -> Result<Vec<VkFence>, vk::Result> {
        let frames_in_flight = std::cmp::max(frames_in_flight, 1);
        let mut fences = Vec::with_capacity(frames_in_flight as usize);
        for _ in 0..frames_in_flight {
            fences.push(VkFence::new(device.clone(), true)?);
        }
        Ok(fences)
    }

    /// Recreates the swapchain in-place. Do NOT call this from a render path instance!
    pub fn recreate_swapchain(
        &mut self,
        platform_interface: &dyn PlatformInterface,
        width: u32,
        height: u32,
    ) -> Result<(), WindowRenderTargetBindingError> {
        tagged_log!(
            "VkGraphics Stage",
            "Resizing swapchain to: {}, {}",
            width,
            height
        );
        unsafe { self.device.device_wait_idle().result()? };
        // Destroy swapchain resources.
        self.in_flight_fences.clear();
        self.images_in_flight.clear();
        Self::destroy_semaphores(&self.device, &mut self.image_available_semaphores);
        Self::destroy_semaphores(&self.device, &mut self.render_finished_semaphores);
        Self::destroy_image_views(&self.device, &mut self.image_views);
        self.images.clear();
        unsafe {
            self.device
                .destroy_swapchain_khr(Some(self.swapchain), None)
        };

        let window = platform_interface
            .get_window(self.window_handle)
            .expect("Must be valid window!");

        let (caps, swapchain, surface_format) = Self::create_swapchain(
            &self.instance,
            &self.device,
            self.physical_device,
            self.surface,
            &self.graphics_options,
            None,
            window,
        )?;

        let extent = Self::select_extent(&caps, window);
        let frames_in_flight = std::cmp::max(self.graphics_options.preferred_frames_in_flight, 1);

        tagged_success!("VkGraphics Stage", "Succesfully built Swapchain.");

        let (mut images, mut image_views) =
            Self::create_images_and_views(&self.device, swapchain, surface_format).map_err(
                |e| unsafe {
                    self.device.destroy_swapchain_khr(Some(swapchain), None);
                    self.instance.destroy_surface_khr(Some(self.surface), None);
                    e
                },
            )?;

        let (mut image_available_semaphores, mut render_finished_semaphores) =
            Self::create_semaphores(&self.device, frames_in_flight).map_err(|e| unsafe {
                Self::destroy_image_views(&self.device, &mut image_views);
                images.clear();
                self.device.destroy_swapchain_khr(Some(swapchain), None);
                self.instance.destroy_surface_khr(Some(self.surface), None);
                e
            })?;

        let in_flight_fences =
            Self::create_fences(&self.device, frames_in_flight).map_err(|e| unsafe {
                Self::destroy_semaphores(&self.device, &mut image_available_semaphores);
                Self::destroy_semaphores(&self.device, &mut render_finished_semaphores);
                Self::destroy_image_views(&self.device, &mut image_views);
                images.clear();
                self.device.destroy_swapchain_khr(Some(swapchain), None);
                self.instance.destroy_surface_khr(Some(self.surface), None);
                e
            })?;

        self.images_in_flight = (0..images.len()).map(|_| vk::Fence::null()).collect();
        self.current_frame_index = 0;
        self.swapchain = swapchain;
        self.images = images;
        self.image_views = image_views;
        self.surface_format = surface_format;
        self.surface_extent = extent;
        self.image_available_semaphores = image_available_semaphores;
        self.render_finished_semaphores = render_finished_semaphores;
        self.in_flight_fences = in_flight_fences;

        Ok(())
    }

    /// Get a reference to the window render target binding's surface.
    pub(crate) fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }

    /// Get a reference to the window render target binding's window handle.
    pub(crate) fn window_handle(&self) -> PlatformWindowHandle {
        self.window_handle
    }

    /// Get a the window render target binding's surface format.
    pub(crate) fn surface_format(&self) -> vk::SurfaceFormatKHR {
        self.surface_format
    }

    /// Get a the window render target binding's surface extent.
    pub(crate) fn surface_extent(&self) -> vk::Extent2D {
        self.surface_extent
    }

    pub(crate) fn image_count(&self) -> u32 {
        self.images.len() as u32
    }

    /// Get a reference to the window render target binding's image views.
    pub(crate) fn image_views(&self) -> &[vk::ImageView] {
        self.image_views.as_slice()
    }

    /// Get a reference to the window render target binding's images.
    pub(crate) fn images(&self) -> &[vk::Image] {
        self.images.as_slice()
    }
}
