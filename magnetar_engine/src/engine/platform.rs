use super::controller::EngineController;

/// Trait that is used to control the state of the game engine and interact with the OS windowing library.
pub trait Platform {
    fn run(&mut self, controller: EngineController);
}
