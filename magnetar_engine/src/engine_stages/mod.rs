use magnetar_utils::dispatch_system::DispatchSystem;

use crate::engine::result::EngineUpdateResult;
use std::{marker::PhantomData, sync::Arc};

pub type UpdateStageConstructor =
    dyn Fn(&mut UpdateStageConstructorInput) -> Box<dyn AnyUpdateStage>;
pub type RenderStageConstructor =
    dyn Fn(&mut RenderStageConstructorInput) -> Box<dyn AnyRenderStage>;

pub struct UpdateStageConstructorInput<'a> {
    _phantom: PhantomData<&'a u8>,
}
impl Default for UpdateStageConstructorInput<'_> {
    fn default() -> Self {
        UpdateStageConstructorInput {
            _phantom: PhantomData::default(),
        }
    }
}

pub struct RenderStageConstructorInput<'a> {
    _phantom: PhantomData<&'a u8>,
}
impl Default for RenderStageConstructorInput<'_> {
    fn default() -> Self {
        RenderStageConstructorInput {
            _phantom: PhantomData::default(),
        }
    }
}

pub struct RenderStageUpdateInput<'a> {
    _phantom: PhantomData<&'a u8>,
}

impl<'a> Default for RenderStageUpdateInput<'a> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
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

    fn update(&mut self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// Render stages run on the main thread. They cannot access regular game data during rendering.
/// The can access data in the update stage, but they can not access `self` while doing so.
pub trait RenderStage: Sized + 'static {
    const IDENTIFIER: &'static str;

    fn update(input: &mut UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: &mut RenderStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Update Stages. Implemented for all T: UpdateStage.
pub trait AnyUpdateStage: Send + 'static {
    fn identifier(&self) -> &'static str;
    fn update(&mut self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult;
}

/// TraitObject trait for Render Stages. Implemented for all T: RenderStage.
pub trait AnyRenderStage: 'static {
    fn identifier(&self) -> &'static str;
    fn update(&self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult;
    fn render(&mut self, input: &mut RenderStageUpdateInput) -> EngineUpdateResult;
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
    fn render(&mut self, input: &mut RenderStageUpdateInput) -> EngineUpdateResult {
        <T as RenderStage>::render(self, input)
    }
    #[inline(always)]
    fn update(&self, input: &mut UpdateStageUpdateInput<'_>) -> EngineUpdateResult {
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
    fn update(&mut self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult {
        <T as UpdateStage>::update(self, input)
    }
}
