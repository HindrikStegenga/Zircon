use std::any::Any;
use crate::engine_stages::{AnyRenderStage, RenderStage, RenderStageUpdateInput, UpdateStageUpdateInput};
use crate::EngineUpdateResult;
use crate::event_manager::{EngineEvent, EngineEventHandler, EventHandlerRegisterer};

pub struct RenderStageContainer<T: RenderStage> {
    stage: T
}

impl<T: RenderStage> From<T> for RenderStageContainer<T> {
    fn from(stage: T) -> Self {
        Self { stage }
    }
}

impl<T: RenderStage> AnyRenderStage for RenderStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as RenderStage>::IDENTIFIER
    }

    fn register_event_handlers(&mut self, registerer: &mut EventHandlerRegisterer) {
        self.stage.register_event_handlers(registerer)
    }

    fn update(&self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        <T as RenderStage>::update(input)
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        self.stage.render(input)
    }
}

struct TestingStage {}
#[derive(Clone)]
struct E {}
impl EngineEvent for E {}
impl<E: EngineEvent> EngineEventHandler<E> for TestingStage {
    fn on_event(&mut self, event: &E) {
        todo!()
    }
}
