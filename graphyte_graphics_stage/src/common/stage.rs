use graphyte_engine::*;
use super::*;

pub struct GraphicsStage {
    entry: ash::Entry,
    instance: ash::Instance,
}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Option<Self> {
        let (entry, instance) = setup_vulkan_instance(&create_info.application_info, &create_info.options)?;
        tagged_success!("Graphics", "Successfully set-up vulkan instance!");

        let target_window = create_info.platform.request_window(640, 480, create_info.application_info.application_name.to_str().unwrap())?;


        Self {
            entry,
            instance
        }.into()
    }
}

impl RenderStage for GraphicsStage {
    const IDENTIFIER: &'static str = "Graphics";

    fn register_message_handlers(&self, _registerer: MessageRegisterer<'_, Self>) {
        ()
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

impl MessageHandler<WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, message: WindowDidOpen) {

    }
}
impl MessageHandler<WindowDidClose> for GraphicsStage {
    fn handle(&mut self, message: WindowDidClose) {

    }
}
impl MessageHandler<WindowDidResize> for GraphicsStage {
    fn handle(&mut self, message: WindowDidResize) {

    }
}