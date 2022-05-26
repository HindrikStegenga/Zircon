pub mod controller;
pub mod create_info;
pub mod engine_states;
pub mod gameloop_timer;
pub mod result;

use crate::platform::*;
use controller::EngineController;
use create_info::EngineCreateInfo;
use engine_states::*;
/// An instance of the game engine.
pub struct Engine {
    state: EngineState,
}

impl From<EngineCreateInfo> for Engine {
    fn from(info: EngineCreateInfo) -> Self {
        Engine {
            state: EngineState::Uninitialized(EngineStateMachine::<Uninitialized>::new(info)),
        }
    }
}

impl Engine {
    /// Runs the engine instance on the given platform.
    pub fn run<P: Platform>(self, platform: P) {
        let controller = EngineController::from(self);
        platform.run(controller);
    }
}
