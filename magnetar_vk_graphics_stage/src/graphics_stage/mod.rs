use std::{ops::Deref, sync::Arc};

use crate::{
    components::{Camera, CameraTargetBinding, CameraType, PerspectiveCamera},
    config::VkGraphicsOptions,
    device::{setup_devices, VkDeviceBindingSet},
    render_paths::{ForwardRenderPath, RenderPathDescriptor, RenderPathType},
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
    pub fn new(create_info: VkGraphicsSystemCreateInfo) -> Result<Self, VkGraphicsSystemError> {
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

        let mut render_path_descriptors = vec![RenderPathDescriptor::new::<ForwardRenderPath>()];

        // Initialize library.
        let library_loader = EntryLoader::new()?;
        tagged_success!("VkGraphics Stage", "Loaded Vulkan library.");

        // Set up vulkan instance.
        let instance = setup_instance(
            &library_loader,
            &create_info.graphics_options,
            &create_info.application_info,
            &mut render_path_descriptors,
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

        // Assign devices to bindings and add path support to each binding.
        let mut bindings: Vec<VkDeviceBindingSet> = {
            let devices = device_config.created_devices;
            let render_path_support = device_config.render_path_support;
            devices
                .into_iter()
                .map(|d| {
                    let mut compatible_paths = vec![];
                    render_path_support.iter().for_each(|(dev, path)| {
                        if *dev == d.physical_device() {
                            compatible_paths.push(path.clone());
                        }
                    });

                    VkDeviceBindingSet::new(d, compatible_paths)
                })
                .collect()
        };

        // All devices have surface support for the first window.
        // Requires at least one path.
        let first_binding = if let Some(b) = bindings.iter_mut().find(|e| {
            e.compatible_paths()
                .iter()
                .find(|p| p.name() == "Forward")
                .is_some()
        }) {
            b
        } else {
            bindings.first_mut().unwrap()
        };

        // Add initial binding
        first_binding.add_window_render_target_binding(
            instance.clone(),
            &create_info.graphics_options,
            default_window.id(),
            device_config.default_render_surface,
        );

        // Bind testing camera
        first_binding.bind_camera_to_first_available_binding(Camera::new(
            CameraType::Perspective(PerspectiveCamera {}),
            CameraTargetBinding::Window(default_window.id()),
            RenderPathType::Forward,
        ));

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
            for binding in device_binding.bindings() {}
        }

        EngineUpdateResult::Ok
    }
}
