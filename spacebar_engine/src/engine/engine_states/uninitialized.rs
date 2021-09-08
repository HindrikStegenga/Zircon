use super::*;
use crate::{engine::gameloop_timer::*, engine_stages::*, *};
use spacebar_utils::dispatch_system::DispatchSystem;
use spacebar_utils::resource_system::*;
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

pub struct Uninitialized {
    render_thread_resources: ResourceSystem,
    update_thread_resources: SendableResourceSystem,
}

impl EngineStateMachine<Uninitialized> {
    #[inline(always)]
    pub fn render_thread_resources(&self) -> &ResourceSystem {
        &self.state.render_thread_resources
    }

    #[inline(always)]
    pub fn render_thread_resources_mut(&mut self) -> &mut ResourceSystem {
        &mut self.state.render_thread_resources
    }

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
        let dispatch_system = Arc::<DispatchSystem>::new(DispatchSystem::new(None));
        let asset_system = match info.asset_system.take() {
            Some(v) => v,
            None => Arc::<AssetSystem>::new(Default::default()),
        };
        let mut shared_resource_system: SendableResourceSystem = Default::default();
        let mut render_thread_resources: ResourceSystem = Default::default();
        let mut update_thread_resources: SendableResourceSystem = Default::default();

        shared_resource_system
            .add_unique_resource(Arc::clone(&dispatch_system))
            .unwrap_or_else(|_| warn!("DispatchSystem was already added. Not adding it."));
        shared_resource_system
            .add_unique_resource(Arc::clone(&asset_system))
            .unwrap_or_else(|_| warn!("AssetSystem was already added. Not adding it."));
        render_thread_resources
            .add_unique_resource(Arc::clone(&dispatch_system))
            .unwrap_or_else(|_| warn!("DispatchSystem was already added. Not adding it."));
        render_thread_resources
            .add_unique_resource(Arc::clone(&asset_system))
            .unwrap_or_else(|_| warn!("AssetSystem was already added. Not adding it."));
        update_thread_resources
            .add_unique_resource(dispatch_system)
            .unwrap_or_else(|_| warn!("DispatchSystem was already added. Not adding it."));
        update_thread_resources
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
                shared_resources: RwLock::new(shared_resource_system),
            },
            state: Uninitialized {
                render_thread_resources: render_thread_resources,
                update_thread_resources,
            },
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
        let (update_stages, render_stages) = {
            let create_info = &uninit.shared.create_info;
            let render_thread_resources = &mut uninit.state.render_thread_resources;
            let update_thread_resources = &mut uninit.state.update_thread_resources;
            let shared_resources = &mut uninit
                .shared
                .shared_resources
                .write()
                .expect("Poison failure");

            let update_stages: Vec<Box<dyn AnyUpdateStage>> = create_info
                .update_stages
                .iter()
                .map(|stage_constructor| {
                    let stage = stage_constructor(UpdateStageConstructorInput::new(
                        interface,
                        update_thread_resources,
                        shared_resources,
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
                        render_thread_resources,
                        shared_resources,
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
                render_thread_resources: uninit.state.render_thread_resources,
                update_thread_resources: uninit.state.update_thread_resources,
            },
        }
    }
}
