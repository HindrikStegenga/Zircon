use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use magnetar_resource_system::*;

pub struct EngineSharedState {
    pub resource_system: ResourceSystem,
    pub create_info: EngineCreateInfo,
    pub internal_resources: EngineInternalResources,
}

pub struct EngineInternalResources {
    pub timings: EngineGameloopTimer,
}
