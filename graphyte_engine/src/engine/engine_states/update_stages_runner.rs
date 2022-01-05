use super::*;
use crate::scene_manager::{Scene, SceneManager};
use crate::{engine::result::*, engine_stages::*};
use graphyte_utils::dispatcher::Dispatcher;
use std::sync::{Arc, Condvar, Mutex};

pub(super) struct UpdateStagesThreadedState {
    scene_manager: SceneManager,
    /// The update stages.
    stages: Vec<Box<dyn AnyUpdateStage>>,
    /// Last result of the threaded loop.
    /// If None, it has not yet been executed.
    last_result: Option<EngineUpdateResult>,
    /// Pre - Update FNs of the render stages.
    render_stage_pre_update_fns: Vec<fn(UpdateStageUpdateInput) -> EngineUpdateResult>,
    /// Post - Update FNs of the render stages.
    render_stage_post_update_fns: Vec<fn(UpdateStageUpdateInput) -> EngineUpdateResult>,
}

pub(super) struct UpdateStagesRunner {
    pub(super) threaded_state: Arc<(Mutex<(bool, UpdateStagesThreadedState)>, Condvar)>,
    dispatch_system: Arc<Dispatcher>,
}

impl UpdateStagesRunner {
    pub fn new(
        scene_manager: SceneManager,
        stages: Vec<Box<dyn AnyUpdateStage>>,
        render_stage_pre_update_fns: Vec<fn(UpdateStageUpdateInput) -> EngineUpdateResult>,
        render_stage_post_update_fns: Vec<fn(UpdateStageUpdateInput) -> EngineUpdateResult>,
        dispatch_system: Arc<Dispatcher>,
    ) -> Self {
        Self {
            threaded_state: Arc::new((
                Mutex::new((
                    false,
                    UpdateStagesThreadedState {
                        scene_manager,
                        stages,
                        last_result: None,
                        render_stage_pre_update_fns,
                        render_stage_post_update_fns,
                    },
                )),
                Condvar::new(),
            )),
            dispatch_system,
        }
    }

    pub fn update(&mut self, shared_state: &mut EngineSharedState) -> EngineUpdateResult {
        // Possibly wait for previous iteration, getting it's message as well.
        let previous_message = self.wait_for_previous_update_completed();

        if previous_message != EngineUpdateResult::Restart {
            // Enqueue new  update job!
            let state = Arc::clone(&self.threaded_state);
            let resources = shared_state.resources.clone();
            let dispatcher = Arc::clone(&self.dispatch_system);
            self.dispatch_system.spawn(move || {
                let &(ref mtx, ref cnd) = &*state;

                let mut guard = mtx.lock().unwrap();
                let threaded_state = &mut guard.1;

                threaded_state.stages.iter_mut().for_each(|s| {
                    s.process_events();
                });

                // Update render stage pre update fns.
                for update_fn in &threaded_state.render_stage_pre_update_fns {
                    let msg = (update_fn)(UpdateStageUpdateInput::new(
                        resources.clone(),
                        dispatcher.clone(),
                        &mut threaded_state.scene_manager,
                    ));
                    if msg == EngineUpdateResult::Ok {
                        continue;
                    };
                    threaded_state.last_result = Some(msg);
                    return;
                }

                // Update
                for system in &mut threaded_state.stages {
                    let msg = system.update(UpdateStageUpdateInput::new(
                        resources.clone(),
                        dispatcher.clone(),
                        &mut threaded_state.scene_manager,
                    ));
                    if msg == EngineUpdateResult::Ok {
                        continue;
                    }
                    threaded_state.last_result = Some(msg);
                    return;
                }

                // Update render stage post update fns.
                for update_fn in &threaded_state.render_stage_post_update_fns {
                    let msg = (update_fn)(UpdateStageUpdateInput::new(
                        resources.clone(),
                        dispatcher.clone(),
                        &mut threaded_state.scene_manager,
                    ));
                    if msg == EngineUpdateResult::Ok {
                        continue;
                    };
                    threaded_state.last_result = Some(msg);
                    return;
                }

                guard.0 = true;
                cnd.notify_one();
            });
        }

        return previous_message;
    }

    fn wait_for_previous_update_completed(&mut self) -> EngineUpdateResult {
        let mut previous_message = EngineUpdateResult::Ok;
        let &(ref mtx, ref cnd) = &*self.threaded_state;
        let mut guard = mtx.lock().unwrap();
        if let Some(_) = &guard.1.last_result {
            while guard.0 == false {
                guard = cnd.wait(guard).unwrap();
            }
            previous_message = *guard.1.last_result.as_ref().unwrap();
            //Guard boolean is true here
            guard.0 = false;
            guard.1.last_result = None;
        }
        previous_message
    }
}
