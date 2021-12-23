pub mod engine;
pub mod engine_stages;
pub mod message_bus;
pub mod platform;
pub mod resource_manager;
pub mod scene_manager;

pub use engine::{create_info::*, result::EngineUpdateResult, Engine};
pub use graphyte_asset_library::asset_system::AssetSystem;
pub use platform::*;

pub use shard_ecs as ecs;

#[cfg(feature = "re_export_utils")]
pub use graphyte_utils::*;

#[cfg(not(feature = "re_export_utils"))]
pub(crate) use graphyte_utils::*;
