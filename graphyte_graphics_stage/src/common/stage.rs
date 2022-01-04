use std::sync::Arc;
use super::instance_setup::*;
use graphyte_engine::*;
use crate::*;

pub struct GraphicsStage {
    entry: ash::Entry,
    instance: Arc<ash::Instance>,
    device: GraphicsDevice,
}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Option<Self> {
        let (entry, instance) = {
            let (entry, instance) = setup_vulkan_instance(&create_info.application_info, &create_info.options)?;
            (entry, Arc::new(instance))
        };
        tagged_success!("Graphics", "Successfully set-up vulkan instance!");

        let device = GraphicsDevice::new(
            GraphicsDeviceCreateInfo {
                instance: Arc::clone(&instance),
                options: &create_info.options
            })?;

        Self { entry, instance, device }.into()
    }
}

impl RenderStage for GraphicsStage {
    const IDENTIFIER: &'static str = "Graphics";

    fn register_message_handlers(&self, mut registerer: MessageRegisterer<'_, Self>) {
        registerer.register::<WindowDidResize>();
        registerer.register::<WindowDidOpen>();
        registerer.register::<WindowDidClose>();
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

impl MessageHandler<WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, message: WindowDidOpen) {}
}
impl MessageHandler<WindowDidClose> for GraphicsStage {
    fn handle(&mut self, message: WindowDidClose) {}
}
impl MessageHandler<WindowDidResize> for GraphicsStage {
    fn handle(&mut self, message: WindowDidResize) {}
}

impl Drop for GraphicsStage {
    fn drop(&mut self) {
        if self.instance.handle() != ash::vk::Instance::null() {
            unsafe { self.instance.destroy_instance(None) }
        }
    }
}