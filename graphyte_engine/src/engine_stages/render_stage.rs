use super::*;
use crate::message_bus::*;
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use std::sync::Arc;

pub type RenderStageConstructor =
    dyn Fn(RenderStageConstructorInput) -> Box<dyn AnyRenderStage> + 'static;

pub struct RenderStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    resources: Arc<EngineResourceManager>,
}

impl<'a> RenderStageConstructorInput<'a> {
    pub fn resources(&self) -> &Arc<EngineResourceManager> {
        &self.resources
    }
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

    fn register_message_handlers(&self, _registerer: RenderMessageRegisterer<'_, Self>) {}
    fn pre_update(_input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    fn post_update(_input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Render Stages. Implemented for all T: RenderStage.
pub trait AnyRenderStage: 'static {
    fn identifier(&self) -> &'static str;
    fn register_message_handlers(&mut self, _registerer: AnyMessageRegisterer<'_>);
    fn process_events(&mut self, input: RenderStageUpdateInput);
    fn get_pre_update_fn(&self) -> fn(input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn get_post_update_fn(&self) -> fn(input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

impl<T: RenderStage> From<T> for Box<dyn AnyRenderStage> {
    fn from(stage: T) -> Self {
        Box::from(RenderStageContainer::from(stage))
    }
}
