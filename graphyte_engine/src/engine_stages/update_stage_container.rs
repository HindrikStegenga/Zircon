use std::any::Any;
use crate::engine_stages::{AnyUpdateStage, UpdateStage, UpdateStageUpdateInput};
use crate::EngineUpdateResult;
use crate::event_manager::EventHandlerRegisterer;

pub struct UpdateStageContainer<T: UpdateStage> {
    stage: T
}

impl<T: UpdateStage> From<T> for UpdateStageContainer<T> {
    fn from(stage: T) -> Self {
        Self { stage }
    }
}

impl<T: UpdateStage> AnyUpdateStage for UpdateStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as UpdateStage>::IDENTIFIER
    }

    fn register_event_handlers(&mut self, registerer: &mut EventHandlerRegisterer) {
        self.stage.register_event_handlers(registerer)
    }

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.update(input)
    }
}

