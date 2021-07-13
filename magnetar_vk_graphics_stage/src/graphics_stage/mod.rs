use std::{ops::Deref, sync::Arc};

use crate::{
    config::VkGraphicsOptions,
    device::{setup_devices, VkDeviceBindingSet},
    render_paths::{ForwardRenderPath, RenderPathDescriptor},
    VkGraphicsSystemCreateInfo, VkGraphicsSystemError, *,
};
use erupt::*;
use magnetar_engine::{engine::create_info::ApplicationInfo, engine_stages::*, EngineUpdateResult};

mod setup_instance;
pub mod vk_device;
pub mod vk_instance;

use setup_instance::*;
use vk_device::*;
use vk_instance::*;

pub struct VkGraphicsStage {
    device_bindings: Vec<VkDeviceBindingSet>,
    instance: VkInstance,
    library_loader: EntryLoader,
    application_info: ApplicationInfo,
    graphics_options: VkGraphicsOptions,
}

impl VkGraphicsStage {
    pub fn new(mut create_info: VkGraphicsSystemCreateInfo) -> Result<Self, VkGraphicsSystemError> {
        // TODO: Remove this part here when events and such are finished.
        let default_window = match create_info.platform_interface.get_windows().first() {
            Some(handle) => create_info
                .platform_interface
                .get_window(*handle)
                .expect("Requires a window!"),
            None => create_info
                .platform_interface
                .request_window(800, 600, "Vulkan Default Window")
                .expect("Requires a window!"),
        };

        let render_path_descriptors = vec![RenderPathDescriptor::from_path::<ForwardRenderPath>()];

        let library_loader = EntryLoader::new()?;
        tagged_success!("VkGraphics Stage", "Loaded Vulkan library.");
        let instance = setup_instance(
            &library_loader,
            &create_info.graphics_options,
            &create_info.application_info,
        )?;
        tagged_success!("VkGraphics Stage", "Created Vulkan Instance.");

        let device_config = unsafe {
            setup_devices(
                default_window,
                &render_path_descriptors,
                &create_info.graphics_options,
                &instance,
            )
        }?;

        if device_config.created_devices.is_empty() {
            return Err(VkGraphicsSystemError::NoSuitableDevicesError);
        }

        let mut bindings: Vec<VkDeviceBindingSet> = device_config
            .created_devices
            .into_iter()
            .map(|d| VkDeviceBindingSet {
                device: d,
                paths: vec![],
                window_bindings: vec![],
            })
            .collect();
        device_config
            .render_path_support
            .into_iter()
            .for_each(|(dev, path)| {
                if let Some(d) = bindings
                    .iter_mut()
                    .find(|e| e.device.physical_device() == dev)
                {
                    d.paths.push(path);
                }
            });

        // All devices have surface support for the first window.
        // Forward Shading must always be supported.
        // TODO: write code to enforce forward shading.
        let first_binding = bindings
            .iter_mut()
            .find(|e| e.paths.iter().find(|p| &(p.name)() == "Forward").is_some())
            .unwrap();

        first_binding.add_window_render_target_binding(
            instance.clone(),
            &create_info.graphics_options,
            default_window.id(),
            device_config.default_render_surface,
        );

        Ok(Self {
            graphics_options: create_info.graphics_options,
            application_info: create_info.application_info,
            library_loader,
            instance,
            device_bindings: bindings,
        })
    }
}

impl RenderStage for VkGraphicsStage {
    const IDENTIFIER: &'static str = "VkGraphics Stage";

    fn update(input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        for device_binding in &self.device_bindings {
            for window_binding in &device_binding.window_bindings {}
        }

        EngineUpdateResult::Ok
    }
}
