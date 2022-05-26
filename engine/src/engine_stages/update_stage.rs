use std::any::Any;

use super::*;
use crate::message_bus::AnyMessageRegisterer;
use crate::{EngineUpdateResult, UpdateMessageRegisterer};

pub type UpdateStageConstructor =
    dyn Fn(UpdateStageConstructorInput) -> Box<dyn AnyUpdateStage> + 'static;

/// Update stages run on a separate thread and update the game's logic.
/// Update stages can issue a request to buffer game data.
pub trait UpdateStage: Sized + Send + 'static {
    const IDENTIFIER: &'static str;
    #[allow(unused_variables)]
    fn register_message_handlers(&self, registerer: UpdateMessageRegisterer<'_, Self>) {}
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;

    /// Executed after the engine is initialized but before running. Unlike other update functions, runs on the main thread.
    #[allow(unused_variables)]
    fn engine_did_initialize(&mut self, input: EngineDidInitInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    #[allow(unused_variables)]
    fn engine_will_suspend(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    #[allow(unused_variables)]
    fn engine_will_resume(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// TraitObject trait for Update Stages. Implemented for all T: UpdateStage.
pub trait AnyUpdateStage: Send + 'static {
    fn identifier(&self) -> &'static str;
    fn process_events(&mut self);
    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>);
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;

    /// Executed after the engine is initialized but before running. Unlike other update functions, runs on the main thread.
    #[allow(unused_variables)]
    fn engine_did_initialize(&mut self, input: EngineDidInitInput) -> EngineUpdateResult;
    #[allow(unused_variables)]
    fn engine_will_suspend(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
    #[allow(unused_variables)]
    fn engine_will_resume(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: UpdateStage> From<T> for Box<dyn AnyUpdateStage> {
    fn from(stage: T) -> Self {
        Box::from(UpdateStageContainer::from(stage))
    }
}
