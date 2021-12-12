use std::sync::Arc;

use graphyte_asset_library::{dispatch_system::DispatchSystem, resource_system::*};

use super::*;
use crate::engine_stages::*;
use crate::*;

pub struct Initialized {
    pub(super) update_stages: Vec<Box<dyn AnyUpdateStage>>,
    pub(super) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Initialized> {
    fn into(self) -> EngineStateMachine<Running> {
        let dispatch_system = match self
            .shared
            .resources
            .get_engine_resource::<DispatchSystem>()
        {
            Some(v) => Arc::clone(&v),
            None => {
                failure!("Internal engine inconsistency! DispatchSystem should be added to the resource systems!");
            }
        };

        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: UpdateStagesRunner::new(
                    self.state.update_stages,
                    dispatch_system,
                ),
                render_stages: self.state.render_stages,
            },
        }
    }
}
