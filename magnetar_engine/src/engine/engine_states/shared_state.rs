use crate::{engine::gameloop_timer::EngineGameloopTimer, EngineCreateInfo};
use magnetar_utils::dispatcher::Dispatcher;

pub struct EngineSharedState {
    pub create_info: EngineCreateInfo,
    pub timings: EngineGameloopTimer,
    pub dispatcher: Dispatcher,
}
