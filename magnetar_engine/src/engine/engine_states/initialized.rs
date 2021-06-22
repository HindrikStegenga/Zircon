use super::*;
use crate::engine_stages::*;

pub struct Initialized {
    pub(crate) update_stages: Vec<Box<dyn AnyUpdateStage>>,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Initialized> {
    fn into(self) -> EngineStateMachine<Running> {
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: UpdateStagesRunner::new(self.state.update_stages),
                render_stages: self.state.render_stages,
            },
        }
    }
}
