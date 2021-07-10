use crate::{config::VkGraphicsOptions, VkGraphicsSystemCreateInfo, VkGraphicsSystemError, *};
use erupt::*;
use magnetar_engine::{engine::create_info::ApplicationInfo, engine_stages::*, EngineUpdateResult};
use std::ffi::CString;

mod setup_instance;
use setup_instance::*;

pub struct VkGraphicsStage {
    graphics_options: VkGraphicsOptions,
    application_info: ApplicationInfo,
    instance_loader: InstanceLoader,
    library_loader: EntryLoader,
}

impl VkGraphicsStage {
    pub fn new(mut create_info: VkGraphicsSystemCreateInfo) -> Result<Self, VkGraphicsSystemError> {
        // TODO: Remove this part here.
        if create_info.platform_interface.get_windows().is_empty() {
            create_info.platform_interface.request_window(800, 600, "");
        }

        let library_loader = EntryLoader::new()?;
        tagged_success!("VkGraphics Stage", "Loaded Vulkan library.");
        let instance_loader = setup_instance(&library_loader, &mut create_info)?;
        tagged_success!("VkGraphics Stage", "Created Vulkan Instance.");
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
