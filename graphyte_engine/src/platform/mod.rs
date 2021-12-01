use crate::engine::controller::EngineController;

pub mod interface;
pub use interface::*;

/// Trait that is used to control the state of the game engine and interact with the OS windowing library.
pub trait Platform {
    fn run(self, controller: EngineController);
}
