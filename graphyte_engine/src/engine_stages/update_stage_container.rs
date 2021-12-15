use std::any::Any;
use crate::engine_stages::{AnyUpdateStage, UpdateStage, UpdateStageUpdateInput};
use crate::EngineUpdateResult;
use crate::message_bus::*;

pub struct UpdateStageContainer<T: UpdateStage> {
    stage: T,
    receivers: Vec<Box<dyn AnyMessageReceiver<T>>>
}

impl<T: UpdateStage> From<T> for UpdateStageContainer<T> {
    fn from(stage: T) -> Self {
        Self { stage, receivers: vec![] }
    }
}

impl<T: UpdateStage> AnyUpdateStage for UpdateStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as UpdateStage>::IDENTIFIER
    }

    fn process_events(&mut self) {
        for receiver in self.receivers.iter_mut() {
            receiver.receive_messages(&mut self.stage);
        }
    }

    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>) {
        self.receivers.clear();
        let registerer = MessageRegisterer::new(registerer, &mut self.receivers);
        self.stage.register_message_handlers(registerer);
    }

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.update(input)
    }
}

