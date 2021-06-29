use super::*;
use crate::{engine::gameloop_timer::*, engine_stages::*, *};
use magnetar_utils::dispatch_system::DispatchSystem;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub struct Uninitialized;

impl EngineStateMachine<Uninitialized> {
    pub fn new(info: EngineCreateInfo) -> Self {
        let instant = Instant::now();
        Self {
            shared: EngineSharedState {
                timings: EngineGameloopTimer {
                    update_tick_rate: info.update_tick_rate,
                    max_skipped_frames: info.max_skipped_frames,
                    max_frame_rate: info.max_frame_rate.clone(),
                    previous_frame_instant: instant,
                    previous_second_instant: instant,
                    last_fixed_update_instant: instant,
                    frame_start_instant: instant,
                    current_delta_time: Duration::new(0, 0),
                    accumulated_time: Duration::new(0, 0),
                    previous_sleep_time: Duration::new(0, 0),
                    negative_sleep_time: Duration::new(0, 0),
                    update_counter: 0,
                    frame_counter: 0,
                    total_sleep_time_last_second: Duration::new(0, 0),
                    total_frame_time_last_second: Duration::new(0, 0),
                    alpha: 0.0,
                },
                create_info: info,
                dispatcher: Arc::new(DispatchSystem::new(Some(1))),
            },
            state: Uninitialized {},
        }
    }
}

impl Into<EngineStateMachine<Initialized>> for EngineStateMachine<Uninitialized> {
    fn into(self) -> EngineStateMachine<Initialized> {
        let mut update_input = UpdateStageConstructorInput::default();
        let mut render_input = RenderStageConstructorInput::default();

        let update_stages: Vec<Box<dyn AnyUpdateStage>> = self
            .shared
            .create_info
            .update_stages
            .iter()
            .map(|stage_constructor| stage_constructor(&mut update_input))
            .collect();

        let render_stages: Vec<Box<dyn AnyRenderStage>> = self
            .shared
            .create_info
            .render_stages
            .iter()
            .map(|stage_constructor| stage_constructor(&mut render_input))
            .collect();

        EngineStateMachine {
            shared: self.shared,
            state: Initialized {
                update_stages,
                render_stages,
            },
        }
    }
}
