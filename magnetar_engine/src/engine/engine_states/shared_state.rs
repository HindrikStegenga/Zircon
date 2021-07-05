use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use magnetar_asset_library::asset_system::AssetSystem;
use magnetar_utils::dispatch_system::DispatchSystem;
use std::sync::Arc;

pub struct EngineSharedState {
    pub create_info: EngineCreateInfo,
    pub resources: EngineCoreResources,
}

pub struct EngineCoreResources {
    pub timings: EngineGameloopTimer,
    pub dispatcher: Arc<DispatchSystem>,
    pub asset_system: AssetSystem,
}
