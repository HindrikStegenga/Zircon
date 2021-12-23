use graphyte_engine::*;
use graphyte_engine::engine_stages::{RenderStage, RenderStageUpdateInput, UpdateStageUpdateInput};
use graphyte_engine::message_bus::{MessageHandler, MessageRegisterer};
use crate::{GraphicsBackend, GraphicsBackendContainer, GraphicsStageCreateInfo};

pub struct GraphicsStage {
    backend: GraphicsBackendContainer
}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Option<Self> {
        Self {
            backend: GraphicsBackendContainer::new(create_info)?
        }.into()
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
        return match &mut self.backend {
            #[cfg(feature = "metal_api")]
            GraphicsBackendContainer::Metal(backend) => { backend.render(input) }
            #[cfg(feature = "vulkan_api")]
            GraphicsBackendContainer::Vulkan(backend) => { backend.render(input) }
            #[cfg(feature = "open_gl_api")]
            GraphicsBackendContainer::OpenGL(backend) => { backend.render(input) }
        }
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