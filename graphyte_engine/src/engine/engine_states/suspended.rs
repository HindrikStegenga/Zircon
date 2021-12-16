use std::sync::Arc;
use graphyte_utils::dispatcher::Dispatcher;
use super::*;
use crate::engine_stages::*;

pub struct Suspended {
    pub(crate) dispatch_system: Arc<Dispatcher>,
    pub(super) update_stages_runner: UpdateStagesRunner,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Suspended> {
    fn into(self) -> EngineStateMachine<Running> {
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                dispatch_system: self.state.dispatch_system,
                update_stages_runner: self.state.update_stages_runner,
                render_stages: self.state.render_stages,
            },
        }
    }
}
