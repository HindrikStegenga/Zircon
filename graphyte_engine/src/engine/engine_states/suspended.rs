use std::sync::Arc;

use graphyte_asset_library::{dispatch_system::DispatchSystem, resource_system::ResourceSystem};

use super::*;
use crate::engine_stages::*;

pub struct Suspended {
    pub(super) update_stages_runner: UpdateStagesRunner,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Suspended> {
    fn into(self) -> EngineStateMachine<Running> {
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: self.state.update_stages_runner,
                render_stages: self.state.render_stages,
            },
        }
    }
}
