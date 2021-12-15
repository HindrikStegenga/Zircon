use std::any::Any;
use crate::message_bus::*;
use crate::engine_stages::{AnyRenderStage, RenderStage, RenderStageUpdateInput, UpdateStageUpdateInput};
use crate::EngineUpdateResult;

pub struct RenderStageContainer<T: RenderStage> {
    stage: T,
    receivers: Vec<Box<dyn AnyMessageReceiver<T>>>
}

impl<T: RenderStage> From<T> for RenderStageContainer<T> {
    fn from(stage: T) -> Self {
        Self { stage, receivers: vec![] }
    }
}

impl<T: RenderStage> AnyRenderStage for RenderStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as RenderStage>::IDENTIFIER
    }

    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>) {
        self.receivers.clear();
        let registerer = MessageRegisterer::new(registerer, &mut self.receivers);
        self.stage.register_message_handlers(registerer);
    }

    fn process_events(&mut self) {
        for receiver in self.receivers.iter_mut() {
            receiver.receive_messages(&mut self.stage);
        }
    }

    fn update(&self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        <T as RenderStage>::update(input)
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        self.stage.render(input)
    }
}