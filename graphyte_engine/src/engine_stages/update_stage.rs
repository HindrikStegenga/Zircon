use super::*;
use crate::message_bus::{AnyMessageRegisterer, MessageRegisterer};
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use graphyte_utils::dispatcher::Dispatcher;
use std::marker::PhantomData;
use std::sync::Arc;

pub type UpdateStageConstructor =
    dyn Fn(UpdateStageConstructorInput) -> Box<dyn AnyUpdateStage> + 'static;

pub struct UpdateStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub resources: Arc<EngineResourceManager>,
}

impl<'a> UpdateStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        resources: Arc<EngineResourceManager>,
    ) -> Self {
        Self {
            platform_interface,
            resources,
        }
    }
}

pub struct UpdateStageUpdateInput<'a> {
    _phantom: PhantomData<&'a ()>,
    resources: Arc<EngineResourceManager>,
    dispatcher: Arc<Dispatcher>,
}

impl<'a> UpdateStageUpdateInput<'a> {
    pub fn resources(&self) -> &Arc<EngineResourceManager> {
        &self.resources
    }
    pub fn dispatcher(&self) -> &Arc<Dispatcher> {
        &self.dispatcher
    }
}

impl<'a> UpdateStageUpdateInput<'a> {
    pub fn new(resources: Arc<EngineResourceManager>, dispatcher: Arc<Dispatcher>) -> Self {
        Self {
            _phantom: Default::default(),
            resources,
            dispatcher,
        }
    }
}

/// Update stages run on a separate thread and update the game's logic.
/// Update stages can issue a request to buffer game data.
pub trait UpdateStage: Sized + Send + 'static {
    const IDENTIFIER: &'static str;

    fn register_message_handlers(&self, _registerer: MessageRegisterer<'_, Self>) {}
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Update Stages. Implemented for all T: UpdateStage.
pub trait AnyUpdateStage: Send + 'static {
    fn identifier(&self) -> &'static str;
    fn process_events(&mut self);
    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>);
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

impl<T: UpdateStage> From<T> for Box<dyn AnyUpdateStage> {
    fn from(stage: T) -> Self {
        Box::from(UpdateStageContainer::from(stage))
    }
}
