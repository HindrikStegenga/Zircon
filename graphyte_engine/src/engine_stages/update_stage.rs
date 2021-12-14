use super::*;
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use graphyte_asset_library::asset_system::AssetSystem;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::event_manager::EventHandlerRegisterer;

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
}

impl<'a> UpdateStageUpdateInput<'a> {
    pub fn new(resources: Arc<EngineResourceManager>) -> Self {
        Self {
            _phantom: Default::default(),
            resources,
        }
    }
}

/// Update stages run on a separate thread and update the game's logic.
/// Update stages can issue a request to buffer game data.
pub trait UpdateStage: Sized + Send + 'static {
    const IDENTIFIER: &'static str;

    fn register_event_handlers(&mut self, _registerer: &mut EventHandlerRegisterer) {}
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Update Stages. Implemented for all T: UpdateStage.
pub trait AnyUpdateStage: Send + 'static {
    fn identifier(&self) -> &'static str;
    fn register_event_handlers(&mut self, registerer: &mut EventHandlerRegisterer);
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

impl<T: UpdateStage> From<T> for Box<dyn AnyUpdateStage> {
    fn from(stage: T) -> Self {
        Box::from(UpdateStageContainer::from(stage))
    }
}