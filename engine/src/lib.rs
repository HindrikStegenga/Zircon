mod engine;
pub mod engine_stages;
pub mod message_bus;
pub mod platform;
pub mod resource_manager;
pub mod scene_manager;

pub use asset_library::asset_system::AssetSystem;
pub use engine::{
    controller::EngineController, create_info::*, result::EngineUpdateResult, Engine,
};
use engine_stages::RenderStageUpdateThreadHandler;
pub use engine_stages::{
    RenderStage, RenderStageConstructor, RenderStageConstructorInput, RenderStageUpdateInput,
    UpdateStage, UpdateStageConstructor, UpdateStageConstructorInput, UpdateStageUpdateInput,
};
pub use message_bus::*;
pub use platform::*;

pub use shard_ecs as ecs;

#[cfg(feature = "re_export_utils")]
pub use utils::*;

#[cfg(not(feature = "re_export_utils"))]
pub(crate) use graphyte_utils::*;
