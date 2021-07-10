use std::sync::Arc;

use magnetar_asset_library::{dispatch_system::DispatchSystem, resource_system::ResourceSystem};

use super::*;
use crate::engine_stages::*;

pub struct Suspended {
    pub(crate) render_thread_resources: ResourceSystem,
    pub(super) update_stages_runner: UpdateStagesRunner,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
    pub(crate) dispatch_system: Arc<DispatchSystem>,
}

impl EngineStateMachine<Suspended> {
    #[inline(always)]
    pub fn render_thread_resources(&self) -> &ResourceSystem {
        &self.state.render_thread_resources
    }

    #[inline(always)]
    pub fn render_thread_resources_mut(&mut self) -> &mut ResourceSystem {
        &mut self.state.render_thread_resources
    }
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Suspended> {
    fn into(self) -> EngineStateMachine<Running> {
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                render_thread_resources: self.state.render_thread_resources,
                update_stages_runner: self.state.update_stages_runner,
                render_stages: self.state.render_stages,
                dispatch_system: self.state.dispatch_system,
            },
        }
    }
}
