pub mod controller;
pub mod create_info;
pub mod engine_states;
pub mod gameloop_timer;
pub mod platform;
pub mod result;

use controller::EngineController;
use create_info::EngineCreateInfo;
use engine_states::*;
use platform::Platform;

/// An instance of the Magnetar game engine.
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
    pub fn run<P: Platform>(self, mut platform: P) {
        let controller = EngineController::from(self);
        platform.run(controller);
    }
}
