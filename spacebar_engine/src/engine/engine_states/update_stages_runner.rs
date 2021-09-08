use super::*;
use crate::{engine::result::*, engine_stages::*};
use spacebar_asset_library::resource_system::SendableResourceSystem;
use spacebar_utils::dispatch_system::DispatchSystem;
use std::sync::{Arc, Condvar, Mutex};

pub(super) struct UpdateStagesThreadedState {
    /// The update stages.
    stages: Vec<Box<dyn AnyUpdateStage>>,
    update_thread_resources: SendableResourceSystem,
    /// Last result of the threaded loop.
    /// If None, it has not yet been executed.
    last_result: Option<EngineUpdateResult>,
}

pub(super) struct UpdateStagesRunner {
    pub(super) threaded_state: Arc<(Mutex<(bool, UpdateStagesThreadedState)>, Condvar)>,
    dispatch_system: Arc<DispatchSystem>,
}

impl UpdateStagesRunner {
    pub fn new(
        stages: Vec<Box<dyn AnyUpdateStage>>,
        update_thread_resources: SendableResourceSystem,
        dispatch_system: Arc<DispatchSystem>,
    ) -> Self {
        Self {
            threaded_state: Arc::new((
                Mutex::new((
                    false,
                    UpdateStagesThreadedState {
                        stages: stages,
                        last_result: None,
                        update_thread_resources,
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
            let dispatcher = Arc::clone(&self.dispatch_system);
            self.dispatch_system.spawn(move || {
                let &(ref mtx, ref cnd) = &*state;

                let mut guard = mtx.lock().unwrap();
                let threaded_state = &mut guard.1;

                // Update
                for system in &mut threaded_state.stages {
                    let msg = system.update(UpdateStageUpdateInput::new(Arc::clone(&dispatcher)));
                    if msg == EngineUpdateResult::Ok {
                        continue;
                    }
                    threaded_state.last_result = Some(msg);
                    return;
                }

                // Completed update (copying state to render system thread)
                //let universe = threaded_state.universe.as_mut();
                //universe.update_render_state(&mut threaded_state.special_fns);

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
