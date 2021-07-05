pub mod engine;
pub mod engine_stages;
pub mod resource_system;

pub use engine::{
    create_info::EngineCreateInfo, platform::Platform, result::EngineUpdateResult, Engine,
};
pub use magnetar_asset_library::asset_system::AssetSystem;

#[cfg(feature = "re_export_logging")]
#[cfg(debug_assertions)]
pub use magnetar_utils::{
    debug_error, debug_failure, debug_log, debug_success, debug_warn, error, failure, log, success,
    warn,
};
#[cfg(feature = "re_export_logging")]
#[cfg(not(debug_assertions))]
pub use magnetar_utils::{error, failure, log, success, warn};

#[cfg(not(feature = "re_export_logging"))]
pub use magnetar_utils::{
    debug_error, debug_failure, debug_log, debug_success, debug_warn, error, failure, log, success,
    warn,
};
