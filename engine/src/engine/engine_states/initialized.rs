use std::sync::Arc;

use asset_library::dispatcher::Dispatcher;

use super::*;
use crate::engine_stages::*;
use crate::resource_manager::ThreadLocalResourceManager;
use crate::scene_manager::SceneManager;
use crate::*;

pub struct Initialized {
    pub(super) update_stages: Vec<Box<dyn AnyUpdateStage>>,
    pub(super) render_stages: Vec<Box<dyn AnyRenderStage>>,
    pub(super) scene_manager: SceneManager,
    pub(super) update_thread_resources: ThreadLocalResourceManager,
    pub(super) render_stage_update_handlers: Vec<Box<dyn AnyRenderStageUpdateThreadHandler>>,
}

impl Into<EngineStateMachine<Running>> for EngineStateMachine<Initialized> {
    fn into(mut self) -> EngineStateMachine<Running> {
        let dispatch_system = match self.shared.resources.get_resource::<Dispatcher>() {
            Some(v) => Arc::clone(&v),
            None => {
                failure!("Internal engine inconsistency! DispatchSystem should be added to the resource systems!");
            }
        };
        EngineStateMachine {
            shared: self.shared,
            state: Running {
                dispatch_system: Arc::clone(&dispatch_system),
                update_stages_runner: UpdateStagesRunner::new(
                    self.state.scene_manager,
                    self.state.update_stages,
                    self.state.render_stage_update_handlers,
                    self.state.update_thread_resources,
                    dispatch_system,
                ),
                render_stages: self.state.render_stages,
            },
        }
    }
}
