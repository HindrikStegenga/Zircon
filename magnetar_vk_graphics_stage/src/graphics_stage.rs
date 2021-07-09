use std::ffi::CString;

use crate::{VkGraphicsSystemCreateInfo, VkGraphicsSystemError, *};
use erupt::*;
use magnetar_engine::{engine_stages::*, EngineUpdateResult};

pub struct VkGraphicsStage {
    create_info: VkGraphicsSystemCreateInfo,
    instance_loader: InstanceLoader,
    library_loader: EntryLoader,
}

impl VkGraphicsStage {
    pub fn new(create_info: VkGraphicsSystemCreateInfo) -> Result<Self, VkGraphicsSystemError> {
        let library_loader = EntryLoader::new()?;
        tagged_success!("VkGraphics Stage", "Loaded Vulkan library.");

        let app_info = vk::ApplicationInfoBuilder::new()
            .api_version(vk::make_api_version(0, 1, 0, 0))
            .application_name(&create_info.application_info.application_name)
            .application_version(vk::make_api_version(
                0,
                create_info.application_info.application_major_version,
                create_info.application_info.application_minor_version,
                create_info.application_info.application_patch_version,
            ))
            .engine_name(&create_info.application_info.engine_name)
            .engine_version(vk::make_api_version(
                0,
                create_info.application_info.engine_major_version,
                create_info.application_info.engine_minor_version,
                create_info.application_info.engine_patch_version,
            ));
        let instance_info = vk::InstanceCreateInfoBuilder::new().application_info(&app_info);

        let instance_loader =
            unsafe { InstanceLoader::new(&library_loader, &instance_info, None)? };
        tagged_success!("VkGraphics Stage", "Created Vulkan Instance.");
        Ok(Self {
            create_info,
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
