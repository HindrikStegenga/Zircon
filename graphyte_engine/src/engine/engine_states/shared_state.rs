use std::sync::{Arc, RwLock};

use crate::resource_manager::EngineResourceManager;
use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use graphyte_utils::resource_system::*;

pub struct EngineSharedState {
    pub resources: Arc<EngineResourceManager>,
    pub create_info: EngineCreateInfo,
    pub internal_resources: EngineInternalResources,
}

pub struct EngineInternalResources {
    pub timings: EngineGameloopTimer,
}
