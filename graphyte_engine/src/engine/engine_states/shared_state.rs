use std::sync::Arc;

use crate::resource_manager::EngineResourceManager;
use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};

pub struct EngineSharedState {
    pub resources: Arc<EngineResourceManager>,
    pub create_info: EngineCreateInfo,
    pub internal_resources: EngineInternalResources,
}

pub struct EngineInternalResources {
    pub timings: EngineGameloopTimer,
}
