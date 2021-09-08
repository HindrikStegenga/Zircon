use spacebar_asset_library::resource_system::SendableResourceSystem;
use spacebar_utils::dispatch_system::DispatchSystem;
use spacebar_utils::resource_system::ResourceSystem;

use crate::{engine::result::EngineUpdateResult, PlatformInterface};
use std::{marker::PhantomData, sync::Arc};

pub type UpdateStageConstructor =
    dyn Fn(UpdateStageConstructorInput) -> Box<dyn AnyUpdateStage> + 'static;
pub type RenderStageConstructor =
    dyn Fn(RenderStageConstructorInput) -> Box<dyn AnyRenderStage> + 'static;

pub struct UpdateStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub update_thread_resources: &'a mut SendableResourceSystem,
    pub shared_resources: &'a mut SendableResourceSystem,
}

impl<'a> UpdateStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        update_thread_resources: &'a mut SendableResourceSystem,
        shared_resources: &'a mut SendableResourceSystem,
    ) -> Self {
        Self {
            platform_interface,
            shared_resources,
            update_thread_resources,
        }
    }
}

pub struct RenderStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub render_thread_resources: &'a mut ResourceSystem,
    pub shared_resources: &'a mut SendableResourceSystem,
}
impl<'a> RenderStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        render_thread_resources: &'a mut ResourceSystem,
        shared_resources: &'a mut SendableResourceSystem,
    ) -> Self {
        RenderStageConstructorInput {
            platform_interface,
            shared_resources,
            render_thread_resources,
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

pub struct UpdateStageUpdateInput<'a> {
    dispatcher: Arc<DispatchSystem>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> UpdateStageUpdateInput<'a> {
    pub fn new(dispatcher: Arc<DispatchSystem>) -> Self {
        Self {
            dispatcher,
            _phantom: Default::default(),
        }
    }
    pub fn dispatcher(&self) -> &Arc<DispatchSystem> {
        &self.dispatcher
    }
}

/// Update stages run on a separate thread and update the game's logic.
/// Update stages can issue a request to buffer game data.
pub trait UpdateStage: Sized + Send + 'static {
    const IDENTIFIER: &'static str;

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// Render stages run on the main thread. They cannot access regular game data during rendering.
/// The can access data in the update stage, but they can not access `self` while doing so.
pub trait RenderStage: Sized + 'static {
    const IDENTIFIER: &'static str;

    fn update(input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
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

impl<T> AnyRenderStage for T
where
    T: RenderStage,
{
    #[inline(always)]
    fn identifier(&self) -> &'static str {
        T::IDENTIFIER
    }

    #[inline(always)]
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        <T as RenderStage>::render(self, input)
    }
    #[inline(always)]
    fn update(&self, input: UpdateStageUpdateInput<'_>) -> EngineUpdateResult {
        <T as RenderStage>::update(input)
    }
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
