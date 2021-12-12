use super::*;
use crate::resource_manager::EngineResourceManager;
use crate::{EngineUpdateResult, PlatformInterface};
use graphyte_asset_library::asset_system::AssetSystem;
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

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Update Stages. Implemented for all T: UpdateStage.
pub trait AnyUpdateStage: Send + 'static {
    fn identifier(&self) -> &'static str;
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Render Stages. Implemented for all T: RenderStage.
pub trait AnyRenderStage: 'static {
    fn identifier(&self) -> &'static str;
    fn update(&self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

impl<T> AnyUpdateStage for T
where
    T: UpdateStage,
{
    #[inline(always)]
    fn identifier(&self) -> &'static str {
        T::IDENTIFIER
    }

    #[inline(always)]
    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        <T as UpdateStage>::update(self, input)
    }
}
