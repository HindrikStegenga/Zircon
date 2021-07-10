use magnetar_asset_library::resource_system::ResourceSystem;

use crate::*;

use super::engine_states::{
    EngineSharedState, EngineState, Initialized, Running, StateMachine, Suspended, Uninitialized,
};

/// A type whose sole purpose is to control the state of the game engine instance.
pub struct EngineController {
    engine: Engine,
}

impl From<Engine> for EngineController {
    fn from(engine: Engine) -> Self {
        Self { engine }
    }
}

impl EngineController {
    pub fn suspend<'a, 'b>(&'a mut self) {
        self.engine.state.suspend();
    }
    pub fn run(&mut self) {
        self.engine.state.run();
    }
    pub fn resume(&mut self) {
        self.engine.state.resume();
    }
    pub fn initialize(&mut self, interface: &mut dyn PlatformInterface) {
        self.engine.state.initialize(interface);
    }
    pub fn reset(&mut self) {
        self.engine.state.reset();
    }

    pub fn render_thread_resources(&self) -> &ResourceSystem {
        match &self.engine.state {
            EngineState::Uninitialized(e) => e.render_thread_resources(),
            EngineState::Initialized(e) => e.render_thread_resources(),
            EngineState::Running(e) => e.render_thread_resources(),
            EngineState::Suspended(e) => e.render_thread_resources(),
            EngineState::Invalid => failure!("Engine state is invalidated."),
        }
    }

    pub fn render_thread_resources_mut(&mut self) -> &mut ResourceSystem {
        match &mut self.engine.state {
            EngineState::Uninitialized(e) => e.render_thread_resources_mut(),
            EngineState::Initialized(e) => e.render_thread_resources_mut(),
            EngineState::Running(e) => e.render_thread_resources_mut(),
            EngineState::Suspended(e) => e.render_thread_resources_mut(),
            EngineState::Invalid => failure!("Engine state is invalidated."),
        }
    }

    pub fn as_running<'b, 'a: 'b>(
        &'a mut self,
        mut handler: impl FnMut(&'b mut StateMachine<Running, EngineSharedState>),
    ) {
        match &mut self.engine.state {
            EngineState::Running(v) => handler(v).into(),
            _ => {
                warn!("Called as_running() on non-running engine state!");
            }
        }
    }

    pub fn as_uninitialized(
        &mut self,
        mut handler: impl FnMut(&mut StateMachine<Uninitialized, EngineSharedState>),
    ) {
        match &mut self.engine.state {
            EngineState::Uninitialized(v) => handler(v),
            _ => {
                warn!("Called as_uninitialized() on non-uninitialized engine state!");
            }
        }
    }

    pub fn as_initialized(
        &mut self,
        mut handler: impl FnMut(&mut StateMachine<Initialized, EngineSharedState>),
    ) {
        match &mut self.engine.state {
            EngineState::Initialized(v) => handler(v),
            _ => {
                warn!("Called as_initialized() on non-uninitialized engine state!");
            }
        }
    }

    pub fn as_suspended(
        &mut self,
        mut handler: impl FnMut(&mut StateMachine<Suspended, EngineSharedState>),
    ) {
        match &mut self.engine.state {
            EngineState::Suspended(v) => handler(v),
            _ => {
                warn!("Called as_suspended() on non-suspended engine state!");
            }
        }
    }
}
