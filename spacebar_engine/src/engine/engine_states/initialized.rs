use std::sync::Arc;

use spacebar_asset_library::{dispatch_system::DispatchSystem, resource_system::*};

use super::*;
use crate::engine_stages::*;
use crate::*;

pub struct Initialized {
    pub(super) render_thread_resources: ResourceSystem,
    pub(super) update_thread_resources: SendableResourceSystem,
    pub(super) update_stages: Vec<Box<dyn AnyUpdateStage>>,
    pub(super) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl EngineStateMachine<Initialized> {
    #[inline(always)]
    pub fn render_thread_resources(&self) -> &ResourceSystem {
        &self.state.render_thread_resources
    }

    #[inline(always)]
    pub fn render_thread_resources_mut(&mut self) -> &mut ResourceSystem {
        &mut self.state.render_thread_resources
    }
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Initialized> {
    fn into(self) -> EngineStateMachine<Running> {
        let dispatch_system = match self
            .state
            .render_thread_resources
            .get_unique_resource::<Arc<DispatchSystem>>()
        {
            Some(v) => Arc::clone(v),
            None => {
                failure!("Internal engine inconsistency! DispatchSystem should be added to the resource systems!");
            }
        };

        EngineStateMachine {
            shared: self.shared,
            state: Running {
                update_stages_runner: UpdateStagesRunner::new(
                    self.state.update_stages,
                    self.state.update_thread_resources,
                    Arc::clone(&dispatch_system),
                ),
                render_stages: self.state.render_stages,
                dispatch_system: dispatch_system,
                render_thread_resources: self.state.render_thread_resources,
            },
        }
    }
}
