use std::sync::Arc;

use magnetar_asset_library::{dispatch_system::DispatchSystem, warn};

use super::*;
use crate::engine_stages::*;

pub struct Initialized {
    pub(crate) update_stages: Vec<Box<dyn AnyUpdateStage>>,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Initialized> {
    fn into(mut self) -> EngineStateMachine<Running> {
        let dispatch_system = match self
            .shared
            .resource_system
            .get_unique_resource_mut::<Arc<DispatchSystem>>()
        {
            Some(v) => Arc::clone(v),
            None => {
                warn!("Internal engine inconsistency! DispatchSystem should be added to the resource system! Added default variant now.");
                let dispatch_system = Arc::new(DispatchSystem::new(None));
                self.shared
                    .resource_system
                    .add_unique_resource(Arc::clone(&dispatch_system));
                dispatch_system
            }
        };

        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: UpdateStagesRunner::new(
                    self.state.update_stages,
                    Arc::clone(&dispatch_system),
                ),
                render_stages: self.state.render_stages,
                dispatch_system: dispatch_system,
            },
        }
    }
}
