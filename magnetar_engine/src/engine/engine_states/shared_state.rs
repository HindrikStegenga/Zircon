use std::sync::RwLock;

use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use magnetar_utils::resource_system::*;

pub struct EngineSharedState {
    pub shared_resources: RwLock<SendableResourceSystem>,
    pub create_info: EngineCreateInfo,
    pub internal_resources: EngineInternalResources,
}

pub struct EngineInternalResources {
    pub timings: EngineGameloopTimer,
}
