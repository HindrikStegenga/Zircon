use super::*;
use crate::{engine::gameloop_timer::*, engine_stages::*, *};
use magnetar_resource_system::*;
use magnetar_utils::dispatch_system::DispatchSystem;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub struct Uninitialized;

impl EngineStateMachine<Uninitialized> {
    pub fn new(info: EngineCreateInfo) -> Self {
        let instant = Instant::now();
        let dispatch_system = Arc::<DispatchSystem>::new(DispatchSystem::new(None));
        let asset_system = Arc::<AssetSystem>::new(Default::default());
        let mut resource_system: ResourceSystem = Default::default();

        resource_system
            .add_unique_resource(dispatch_system)
            .unwrap_or_else(|_| warn!("DispatchSystem was already added. Not adding it."));
        resource_system
            .add_unique_resource(asset_system)
            .unwrap_or_else(|_| warn!("AssetSystem was already added. Not adding it."));

        Self {
            shared: EngineSharedState {
                internal_resources: EngineInternalResources {
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
                },
                create_info: info,
                resource_system,
            },
            state: Uninitialized {},
        }
    }
}

impl Into<EngineStateMachine<Initialized>> for EngineStateMachine<Uninitialized> {
    fn into(mut self) -> EngineStateMachine<Initialized> {
        let create_info = &self.shared.create_info;
        let resource_system = &mut self.shared.resource_system;
        let shared_resources = &mut self.shared.internal_resources;

        let update_stages: Vec<Box<dyn AnyUpdateStage>> = create_info
            .update_stages
            .iter()
            .map(|stage_constructor| {
                stage_constructor(UpdateStageConstructorInput::new(
                    shared_resources,
                    resource_system,
                ))
            })
            .collect();

        let render_stages: Vec<Box<dyn AnyRenderStage>> = create_info
            .render_stages
            .iter()
            .map(|stage_constructor| stage_constructor(RenderStageConstructorInput::default()))
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
