use crate::{
    config::VkGraphicsOptions,
    device::setup_devices,
    render_paths::{ForwardRenderPath, RenderPathDescriptor},
    VkGraphicsSystemCreateInfo, VkGraphicsSystemError, *,
};
use erupt::*;
use magnetar_engine::{engine::create_info::ApplicationInfo, engine_stages::*, EngineUpdateResult};

mod setup_instance;
use setup_instance::*;

pub struct VkGraphicsStage {
    instance_loader: InstanceLoader,
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
        let instance_loader = setup_instance(
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
                &instance_loader,
            )
        }?;

        Ok(Self {
            graphics_options: create_info.graphics_options,
            application_info: create_info.application_info,
            library_loader,
            instance_loader,
        })
    }
}

impl RenderStage for VkGraphicsStage {
    const IDENTIFIER: &'static str = "VkGraphics Stage";

    fn update(input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

impl Drop for VkGraphicsStage {
    fn drop(&mut self) {
        unsafe { self.instance_loader.destroy_instance(None) };
    }
}
