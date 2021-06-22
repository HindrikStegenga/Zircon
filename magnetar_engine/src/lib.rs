pub mod engine;
pub mod engine_stages;

pub use engine::{
    create_info::EngineCreateInfo, platform::Platform, result::EngineUpdateResult, Engine,
};

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
pub(crate) use magnetar_utils::{
    debug_error, debug_failure, debug_log, debug_success, debug_warn, error, failure, log, success,
    warn,
};
