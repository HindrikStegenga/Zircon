mod engine;
pub mod engine_stages;
pub mod message_bus;
pub mod platform;
pub mod resource_manager;
pub mod scene_manager;

pub use engine::{
    controller::EngineController, create_info::*, result::EngineUpdateResult, Engine,
};
pub use engine_stages::{
    RenderStage, RenderStageConstructor, RenderStageConstructorInput, RenderStageUpdateInput,
    UpdateStage, UpdateStageConstructor, UpdateStageConstructorInput, UpdateStageUpdateInput,
};
pub use message_bus::*;
pub use platform::*;

pub use shard_ecs as ecs;

#[allow(dead_code)]
pub(crate) const IDENTIFIER: &'static str = "Engine";
