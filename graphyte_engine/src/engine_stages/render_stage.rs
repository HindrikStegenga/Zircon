use super::*;
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use std::sync::Arc;

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

    fn update(input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

impl<T> AnyRenderStage for T
where
    T: RenderStage,
{
    #[inline(always)]
    fn identifier(&self) -> &'static str {
        T::IDENTIFIER
    }

    #[inline(always)]
    fn update(&self, input: UpdateStageUpdateInput<'_>) -> EngineUpdateResult {
        <T as RenderStage>::update(input)
    }
    #[inline(always)]
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        <T as RenderStage>::render(self, input)
    }
}
