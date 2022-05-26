use super::*;
use crate::engine_stages::inputs::RenderStageConstructorInput;
use crate::message_bus::*;
use crate::resource_manager::ThreadLocalResourceManager;
use crate::EngineUpdateResult;
use std::any::Any;

pub type RenderStageConstructor =
    dyn Fn(RenderStageConstructorInput) -> Box<dyn AnyRenderStage> + 'static;

/// Bundles the available mutable state that can be modified during creation of a render stage update thread handler.
pub struct RenderStageUpdateThreadHandlerCreateInfo<'a> {
    resources: &'a mut ThreadLocalResourceManager,
}

impl<'a> RenderStageUpdateThreadHandlerCreateInfo<'a> {
    pub fn new(resources: &'a mut ThreadLocalResourceManager) -> Self {
        RenderStageUpdateThreadHandlerCreateInfo { resources }
    }
    pub fn resources(&mut self) -> &mut ThreadLocalResourceManager {
        self.resources
    }
}

/// Deals with logic from a render stage within the update thread.
/// To send messages to the render thread, use the message bus or a channel.
pub trait RenderStageUpdateThreadHandler: Sized + Send + 'static {
    /// Register messages which need to be received on the update thread here.
    fn register_message_handlers(&self, _registerer: UpdateMessageRegisterer<'_, Self>) {}
    /// Executed for each render stage before the game state is updated.
    /// This is executed on the update thread.
    fn pre_update(&mut self, _input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    /// Executed for each render stage after the game state is updated.
    /// /// This is executed on the update thread.
    fn post_update(&mut self, _input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

pub trait AnyRenderStageUpdateThreadHandler: Send {
    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>);
    fn process_events(&mut self, input: UpdateStageUpdateInput);
    fn pre_update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
    fn post_update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// Render stages run on the main thread. They cannot access regular game data during rendering.
/// They can access data in the update stage, but they can not access `self` while doing so.
/// Everything related to logic on the update thread is performed through a handler object.
pub trait RenderStage: Sized + 'static {
    const IDENTIFIER: &'static str;
    type UpdateThreadHandler: RenderStageUpdateThreadHandler;

    /// Executed after the engine is initialized but before running. Runs on the main thread.
    #[allow(unused_variables)]
    fn engine_did_initialize(&mut self, input: EngineDidInitInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    #[allow(unused_variables)]
    fn engine_will_suspend(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    #[allow(unused_variables)]
    fn engine_will_resume(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }

    #[allow(unused_variables)]
    fn register_message_handlers(&self, registerer: RenderMessageRegisterer<'_, Self>) {}
    fn create_update_thread_handler(
        &mut self,
        create_info: RenderStageUpdateThreadHandlerCreateInfo<'_>,
    ) -> Self::UpdateThreadHandler;
    /// Runs on the main thread right after the update thread finished a single update.
    #[allow(unused_variables)]
    fn update_thread_did_run(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
    /// Is called on the main thread.
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// TraitObject trait for Render Stages. Implemented for all T: RenderStage.
pub trait AnyRenderStage: 'static {
    fn identifier(&self) -> &'static str;
    fn register_message_handlers(&mut self, _registerer: AnyMessageRegisterer<'_>);
    fn create_update_thread_handler(
        &mut self,
        create_info: RenderStageUpdateThreadHandlerCreateInfo<'_>,
        registerer: AnyMessageRegisterer<'_>,
    ) -> Box<dyn AnyRenderStageUpdateThreadHandler>;

    /// Executed after the engine is initialized but before running. Runs on the main thread.
    #[allow(unused_variables)]
    fn engine_did_initialize(&mut self, input: EngineDidInitInput) -> EngineUpdateResult;
    #[allow(unused_variables)]
    fn engine_will_suspend(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
    #[allow(unused_variables)]
    fn engine_will_resume(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;

    fn process_events(&mut self, input: RenderStageUpdateInput);
    fn update_thread_did_run(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: RenderStage> From<T> for Box<dyn AnyRenderStage> {
    fn from(stage: T) -> Self {
        Box::from(RenderStageContainer::from(stage))
    }
}
