pub mod engine;
pub mod engine_stages;
pub mod platform;

pub use engine::{create_info::EngineCreateInfo, result::EngineUpdateResult, Engine};
pub use magnetar_asset_library::asset_system::AssetSystem;
pub use platform::*;

#[cfg(feature = "re_export_utils")]
pub use magnetar_utils::*;

#[cfg(not(feature = "re_export_utils"))]
pub(crate) use magnetar_utils::*;
