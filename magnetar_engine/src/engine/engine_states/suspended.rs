use std::sync::Arc;

use magnetar_asset_library::dispatch_system::DispatchSystem;

use super::*;
use crate::engine_stages::*;

pub struct Suspended {
    pub(super) update_stages_runner: UpdateStagesRunner,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
    pub(crate) dispatch_system: Arc<DispatchSystem>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Suspended> {
    fn into(self) -> EngineStateMachine<Running> {
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: self.state.update_stages_runner,
                render_stages: self.state.render_stages,
                dispatch_system: self.state.dispatch_system,
            },
        }
    }
}
