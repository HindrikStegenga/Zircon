use super::*;
use crate::scene_manager::SceneManager;
use crate::{engine::gameloop_timer::*, engine_stages::*, resource_manager::*, *};
use graphyte_utils::dispatch_system::DispatchSystem;
use graphyte_utils::resource_system::*;
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

pub struct Uninitialized {}

impl EngineStateMachine<Uninitialized> {
    pub fn new(mut info: EngineCreateInfo) -> Self {
        log!(
            "Initializing {:#?} version {:#?}.{:#?}.{:#?}",
            info.application_info.engine_name,
            info.application_info.engine_major_version,
            info.application_info.engine_minor_version,
            info.application_info.engine_patch_version
        );
        log!(
            "Executing application: {:#?} version {:#?}.{:#?}.{:#?}",
            info.application_info.application_name,
            info.application_info.application_major_version,
            info.application_info.application_minor_version,
            info.application_info.application_patch_version
        );

        let instant = Instant::now();
        let resources = EngineResourceManager::default();
        let dispatch_system = DispatchSystem::new(None);
        let asset_system = match info.asset_system.take() {
            Some(v) => v,
            None => Default::default(),
        };
        resources.add_engine_resource(asset_system);
        resources.add_engine_resource(dispatch_system);
        resources.add_engine_resource(SceneManager::default());

        Self {
            shared: EngineSharedState {
                resources: Arc::new(resources),
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
            },
            state: Uninitialized {},
        }
    }
}

impl Into<EngineStateMachine<Initialized>>
    for (
        EngineStateMachine<Uninitialized>,
        &mut dyn PlatformInterface,
    )
{
    fn into(self) -> EngineStateMachine<Initialized> {
        let (mut uninit, interface) = self;

        log!("Initializing game engine...");
        let (mut update_stages, mut render_stages) = {
            let create_info = &uninit.shared.create_info;
            let update_stages: Vec<Box<dyn AnyUpdateStage>> = create_info
                .update_stages
                .iter()
                .map(|stage_constructor| {
                    let stage = stage_constructor(UpdateStageConstructorInput::new(
                        interface,
                        Arc::clone(&uninit.shared.resources),
                    ));
                    success!("Constructed update stage: {}", stage.identifier());
                    stage
                })
                .collect();

            let render_stages: Vec<Box<dyn AnyRenderStage>> = create_info
                .render_stages
                .iter()
                .map(|stage_constructor| {
                    let stage = stage_constructor(RenderStageConstructorInput::new(
                        interface,
                        Arc::clone(&uninit.shared.resources),
                    ));
                    success!("Constructed render stage: {}", stage.identifier());
                    stage
                })
                .collect();
            (update_stages, render_stages)
        };
        success!("Initialized engine.");
        EngineStateMachine {
            shared: uninit.shared,
            state: Initialized {
                update_stages,
                render_stages,
            },
        }
    }
}
