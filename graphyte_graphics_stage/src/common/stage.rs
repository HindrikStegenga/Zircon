use graphyte_engine::*;
use graphyte_engine::engine_stages::{RenderStage, RenderStageUpdateInput, UpdateStageUpdateInput};
use graphyte_engine::message_bus::{MessageHandler, MessageRegisterer};
use crate::GraphicsStageCreateInfo;

pub struct GraphicsStage {

}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Self {
        Self {}
    }
}

impl RenderStage for GraphicsStage {
    const IDENTIFIER: &'static str = "GraphicsStage";

    fn register_message_handlers(&self, mut registerer: MessageRegisterer<'_, Self>) {
        registerer.register::<WindowDidOpen>();
        registerer.register::<WindowDidClose>();
        registerer.register::<WindowDidResize>();
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        todo!()
    }
}

impl MessageHandler<WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, message: WindowDidOpen) {
        todo!()
    }
}

impl MessageHandler<WindowDidClose> for GraphicsStage {
    fn handle(&mut self, message: WindowDidClose) {
        todo!()
    }
}

impl MessageHandler<WindowDidResize> for GraphicsStage {
    fn handle(&mut self, message: WindowDidResize) {
        todo!()
    }
}