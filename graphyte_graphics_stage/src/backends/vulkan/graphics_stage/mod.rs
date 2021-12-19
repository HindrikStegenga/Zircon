use std::{ops::Deref, sync::Arc};

use crate::{
    *, vulkan::*
};
use erupt::*;
use graphyte_engine::*;
use graphyte_engine::message_bus::{MessageBus, MessageHandler, MessageRegisterer};
use graphyte_engine::{engine::create_info::ApplicationInfo, engine_stages::*, EngineUpdateResult};

mod setup_instance;
pub mod vk_device;
pub mod vk_instance;

use setup_instance::*;
use vk_instance::*;
use crate::backends::vulkan::{ForwardRenderPath, RenderPathDescriptor, RenderPathType};
use crate::backends::vulkan::{setup_devices, VkDeviceBindingSet};
use crate::backends::vulkan::{Camera, CameraTargetBinding, CameraType, PerspectiveCamera, VkGraphicsOptions};

pub struct VkGraphicsStage {
    device_bindings: Vec<VkDeviceBindingSet>,
    instance: VkInstance,
    library_loader: EntryLoader,
    application_info: ApplicationInfo,
    graphics_options: VkGraphicsOptions,
}

impl VkGraphicsStage {
    pub fn new(create_info: VkGraphicsSystemCreateInfo) -> Result<Self, VkGraphicsSystemError> {
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
}

impl RenderStage for VkGraphicsStage {
    const IDENTIFIER: &'static str = "VkGraphics Stage";

    fn register_message_handlers(&self, mut _registerer: MessageRegisterer<'_, Self>) {
        _registerer.register::<TestMessage>();
    }

    fn pre_update(input: UpdateStageUpdateInput) -> EngineUpdateResult {
        let bus = input.resources().get_engine_resource::<MessageBus>().unwrap();
        let sender = bus.get_sender::<TestMessage>().unwrap();
        sender.send(TestMessage {});
        EngineUpdateResult::Ok
    }

    fn render(&mut self, mut input: RenderStageUpdateInput) -> EngineUpdateResult {
        self.device_bindings.iter_mut().for_each(|e| {
            e.render(&mut input);
        });
        EngineUpdateResult::Ok
    }
}

#[derive(Debug, Clone)]
struct TestMessage {}
impl MessageHandler<TestMessage> for VkGraphicsStage {
    fn handle(&mut self, _message: TestMessage) {}
}
