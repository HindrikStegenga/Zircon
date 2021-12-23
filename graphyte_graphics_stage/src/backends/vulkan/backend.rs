use std::sync::Arc;
use crate::vulkan::*;
use crate::vulkan::vk_instance::*;
use crate::vulkan::setup_devices;
use erupt::*;
use graphyte_engine::*;
use crate::vulkan::graphics_stage::setup_instance::setup_instance;
use graphyte_engine::engine_stages::RenderStageUpdateInput;
use crate::{GraphicsBackend, GraphicsBackendCreateInfo};

pub struct VulkanRenderBackend {
    device_bindings: Vec<VkDeviceBindingSet>,
    instance: VkInstance,
    library_loader: EntryLoader,
    application_info: ApplicationInfo,
    graphics_options: VkGraphicsOptions,
}

impl GraphicsBackend for VulkanRenderBackend {
    const API_IDENTIFIER: &'static str = "vulkan";
    type GraphicsOptions = VkGraphicsOptions;
    type ErrorType = VkGraphicsSystemError;

    fn new(create_info: GraphicsBackendCreateInfo<'_, Self::GraphicsOptions>) -> Result<Self, Self::ErrorType> {
        let asset_system = Arc::clone(&create_info.asset_system);
        // TODO: Remove this part here when events and such are finished.
        let default_window_handle = match create_info.platform_interface.get_windows().first() {
            Some(handle) => *handle,
            None => create_info
                .platform_interface
                .request_window(800, 600, "Vulkan Default Window")
                .expect("Requires a window!")
                .handle(),
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
                create_info
                    .platform_interface
                    .get_window(default_window_handle)
                    .unwrap(),
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

                    VkDeviceBindingSet::new(d, compatible_paths, Arc::clone(&asset_system))
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
            create_info.platform_interface,
            default_window_handle,
            device_config.default_render_surface,
        );

        // Bind testing camera
        first_binding.bind_camera_to_first_available_binding(Camera::new(
            CameraType::Perspective(PerspectiveCamera {}),
            CameraTargetBinding::Window(default_window_handle),
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

    fn render(&mut self, mut input: RenderStageUpdateInput) -> EngineUpdateResult {
        self.device_bindings.iter_mut().for_each(|e| {
            e.render(&mut input);
        });
        EngineUpdateResult::Ok
    }
}