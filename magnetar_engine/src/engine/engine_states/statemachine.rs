use super::*;
use crate::*;

pub struct StateMachine<State, SharedState = ()> {
    pub shared: SharedState,
    pub state: State,
}

impl<S> From<S> for StateMachine<S, ()> {
    fn from(state: S) -> Self {
        return Self { shared: (), state };
    }
}

impl<State, SharedState> From<(State, SharedState)> for StateMachine<State, SharedState> {
    fn from(state: (State, SharedState)) -> Self {
        let (state, shared) = state;
        return Self { shared, state };
    }
}

pub type EngineStateMachine<T> = StateMachine<T, EngineSharedState>;
pub enum EngineState {
    Uninitialized(EngineStateMachine<Uninitialized>),
    Initialized(EngineStateMachine<Initialized>),
    Running(EngineStateMachine<Running>),
    Suspended(EngineStateMachine<Suspended>),
    Invalid,
}

impl EngineState {
    pub fn initialize(&mut self) {
        *self = match std::mem::replace(self, EngineState::Invalid) {
            EngineState::Uninitialized(s) => {
                let s = EngineState::Initialized(s.into());
                log!("EngineState changed: Initialized");
                s
            }
            s => {
                warn!("Cannot initialize game engine while not in Uninitialized state!");
                s
            }
        }
    }

    pub fn run(&mut self) {
        *self = match std::mem::replace(self, EngineState::Invalid) {
            EngineState::Initialized(s) => {
                let s = EngineState::Running(s.into());
                log!("EngineState changed: Running");
                s
            }
            s => {
                warn!("Cannot run game engine while not in Initialized state!");
                s
            }
        }
    }

    pub fn suspend(&mut self) {
        *self = match std::mem::replace(self, EngineState::Invalid) {
            EngineState::Running(s) => {
                let s = EngineState::Suspended(s.into());
                log!("EngineState changed: Suspended");
                s
            }
            s => {
                warn!("Cannot suspend game engine while not in Running state!");
                s
            }
        }
    }

    pub fn reset(&mut self) {
        *self = match std::mem::replace(self, EngineState::Invalid) {
            EngineState::Uninitialized(s) => {
                EngineState::Uninitialized(EngineStateMachine::<Uninitialized>::new(
                    s.shared.create_info,
                ))
            }
            EngineState::Initialized(s) => {
                EngineState::Uninitialized(EngineStateMachine::<Uninitialized>::new(
                    s.shared.create_info,
                ))
            }
            EngineState::Running(s) => {
                EngineState::Uninitialized(EngineStateMachine::<Uninitialized>::new(
                    s.shared.create_info,
                ))
            }
            EngineState::Suspended(s) => {
                EngineState::Uninitialized(EngineStateMachine::<Uninitialized>::new(
                    s.shared.create_info,
                ))
            }
            EngineState::Invalid => {
                failure!("Cannot take shared state on invalid enum!");
            }
        }
    }

    pub fn resume(&mut self) {
        *self = match std::mem::replace(self, EngineState::Invalid) {
            EngineState::Suspended(s) => {
                let s = EngineState::Running(s.into());
                log!("EngineState changed: Running");
                s
            }
            s => {
                warn!("Cannot resume game engine while not in Suspended state!");
                s
            }
        }
    }
}
