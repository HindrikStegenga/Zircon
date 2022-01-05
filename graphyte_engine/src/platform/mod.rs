use crate::engine::controller::EngineController;
use std::sync::Arc;

mod interface;
mod messages;

pub use interface::*;
pub use messages::*;

/// Trait that is used to control the state of the game engine and interact with the OS windowing library.
pub trait Platform {
    fn run(self, controller: EngineController);
}
