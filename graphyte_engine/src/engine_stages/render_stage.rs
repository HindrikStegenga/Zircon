use super::*;
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use std::sync::Arc;
use crate::event_manager::EventHandlerRegisterer;

pub type RenderStageConstructor =
    dyn Fn(RenderStageConstructorInput) -> Box<dyn AnyRenderStage> + 'static;

pub struct RenderStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub resources: Arc<EngineResourceManager>,
}
impl<'a> RenderStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        resources: Arc<EngineResourceManager>,
    ) -> Self {
        RenderStageConstructorInput {
            platform_interface,
            resources,
        }
    }
}

pub struct RenderStageUpdateInput<'a> {
    pub platform: &'a mut dyn PlatformInterface,
}

impl<'a> RenderStageUpdateInput<'a> {
    pub fn new(platform: &'a mut dyn PlatformInterface) -> Self {
        Self { platform }
    }
}
/// Render stages run on the main thread. They cannot access regular game data during rendering.
/// They can access data in the update stage, but they can not access `self` while doing so.
pub trait RenderStage: Sized + 'static {
    const IDENTIFIER: &'static str;

    fn register_event_handlers(&mut self, _registerer: &mut EventHandlerRegisterer) {}
    fn update(input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Render Stages. Implemented for all T: RenderStage.
pub trait AnyRenderStage: 'static {
    fn identifier(&self) -> &'static str;
    fn register_event_handlers(&mut self, registerer: &mut EventHandlerRegisterer);
    fn update(&self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

impl<T: RenderStage> From<T> for Box<dyn AnyRenderStage> {
    fn from(stage: T) -> Self {
        Box::from(RenderStageContainer::from(stage))
    }
}