use crate::engine_stages::{
    AnyUpdateStage, EngineDidInitInput, UpdateStage, UpdateStageUpdateInput,
};
use crate::message_bus::*;
use crate::{EngineUpdateResult, RenderStageUpdateInput};
use std::any::Any;
use std::marker::PhantomData;

pub struct UpdateStageContainer<T: UpdateStage> {
    stage: T,
    receivers: Vec<Box<dyn AnyUpdateMessageReceiver<T>>>,
}

pub struct UpdateStageMessageContext<'a> {
    phantom: PhantomData<fn(&'a u32)>,
}
impl<'a> Default for UpdateStageMessageContext<'a> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: UpdateStage> From<T> for UpdateStageContainer<T> {
    fn from(stage: T) -> Self {
        Self {
            stage,
            receivers: vec![],
        }
    }
}

impl<T: UpdateStage> AnyUpdateStage for UpdateStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as UpdateStage>::IDENTIFIER
    }

    fn process_events(&mut self) {
        for receiver in self.receivers.iter_mut() {
            receiver.receive_messages(
                &mut self.stage,
                &mut UpdateStageMessageContext {
                    phantom: Default::default(),
                },
            );
        }
    }

    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>) {
        self.receivers.clear();
        let registerer = UpdateMessageRegisterer::new(registerer, &mut self.receivers);
        self.stage.register_message_handlers(registerer);
    }

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.update(input)
    }

    fn engine_did_initialize(&mut self, input: EngineDidInitInput) -> EngineUpdateResult {
        self.stage.engine_did_initialize(input)
    }

    fn engine_will_suspend(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.engine_will_suspend(input)
    }

    fn engine_will_resume(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.engine_will_resume(input)
    }

    fn stage_as_any(&self) -> &dyn Any {
        T::stage_as_any(&self.stage)
    }

    fn stage_as_any_mut(&mut self) -> &mut dyn Any {
        T::stage_as_any_mut(&mut self.stage)
    }
}
