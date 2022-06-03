use super::*;
use crate::{engine::result::*, engine_stages::*, PlatformInterface};
use std::sync::Arc;
use std::time::*;
use utils::dispatcher::Dispatcher;
use utils::split_view::*;

pub struct Running {
    pub(crate) dispatch_system: Arc<Dispatcher>,
    pub(super) update_stages_runner: UpdateStagesRunner,
    pub(crate) render_stages: Vec<Box<dyn AnyRenderStage>>,
}

impl Into<EngineStateMachine<Suspended>> for EngineStateMachine<Running> {
    fn into(self) -> EngineStateMachine<Suspended> {
        EngineStateMachine {
            shared: self.shared,
            state: Suspended {
                dispatch_system: self.state.dispatch_system,
                update_stages_runner: self.state.update_stages_runner,
                render_stages: self.state.render_stages,
            },
        }
    }
}

impl EngineStateMachine<Running> {
    pub fn tick(&mut self, interface: &mut dyn PlatformInterface) -> EngineUpdateResult {
        self.shared.internal_resources.timings.frame_start();

        let tick_rate = self.shared.internal_resources.timings.update_tick_rate;
        let alpha = self.shared.internal_resources.timings.alpha;

        let fixed_update_step_duration = Duration::from_millis(1000)
            / (self.shared.internal_resources.timings.update_tick_rate as u32);

        {
            let frame_counter_past_second = self.shared.internal_resources.timings.frame_counter;
            let update_counter_past_second = self.shared.internal_resources.timings.update_counter;
            // Process events on the render stage thread.
            self.state.render_stages.iter_mut().for_each(|s| {
                s.process_events(RenderStageUpdateInput::new(
                    interface,
                    tick_rate,
                    alpha,
                    frame_counter_past_second,
                    update_counter_past_second,
                ));
            });
        }

        // Trigger the update thread if necessary.
        let mut n_loops = 0;
        while self.shared.internal_resources.timings.accumulated_time >= fixed_update_step_duration
            && n_loops < (1 + self.shared.internal_resources.timings.max_skipped_frames)
        {
            match self.state.update_stages_runner.update(&mut self.shared) {
                EngineUpdateResult::Ok => {}
                result => {
                    return result;
                }
            }

            self.shared.internal_resources.timings.accumulated_time -= fixed_update_step_duration;
            n_loops += 1;
            self.shared.internal_resources.timings.update_counter += 1;
            self.shared
                .internal_resources
                .timings
                .last_fixed_update_instant =
                self.shared.internal_resources.timings.frame_start_instant;

            let frame_counter_past_second = self.shared.internal_resources.timings.frame_counter;
            let update_counter_past_second = self.shared.internal_resources.timings.update_counter;

            if let Err(e) = SplitViewMut::for_each_until_error(
                &mut self.state.render_stages,
                |mut split_view| match split_view.item_mut().update_thread_did_run(
                    RenderStageUpdateInput::new(
                        interface,
                        tick_rate,
                        alpha,
                        frame_counter_past_second,
                        update_counter_past_second,
                    ),
                ) {
                    EngineUpdateResult::Ok => Ok(()),
                    result => Err(result),
                },
            ) {
                return e;
            };
        }

        // Trigger the render thread.
        let frame_counter_past_second = self.shared.internal_resources.timings.frame_counter;
        let update_counter_past_second = self.shared.internal_resources.timings.update_counter;

        if let Err(e) =
            SplitViewMut::for_each_until_error(&mut self.state.render_stages, |mut split_view| {
                let (before, item, after) = split_view.components_mut();
                let _manager = RenderStageManager::from_slices(before, after);
                match item.render(RenderStageUpdateInput::new(
                    interface,
                    tick_rate,
                    alpha,
                    frame_counter_past_second,
                    update_counter_past_second,
                )) {
                    EngineUpdateResult::Ok => Ok(()),
                    result => Err(result),
                }
            })
        {
            return e;
        }
        self.shared.internal_resources.timings.frame_end();

        EngineUpdateResult::Ok
    }
}
