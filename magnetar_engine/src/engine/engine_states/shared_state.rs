use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use magnetar_utils::dispatch_system::DispatchSystem;
use std::sync::Arc;

pub struct EngineSharedState {
    pub create_info: EngineCreateInfo,
    pub timings: EngineGameloopTimer,
    pub dispatcher: Arc<DispatchSystem>,
}
