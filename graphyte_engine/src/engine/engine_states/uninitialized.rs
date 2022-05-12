use super::*;
use crate::message_bus::{AnyMessageRegisterer, MessageBusBuilder, MessageHandlerType};
use crate::scene_manager::SceneManager;
use crate::{engine::gameloop_timer::*, engine_stages::*, resource_manager::*, *};
use graphyte_utils::dispatcher::Dispatcher;
use std::{
    sync::Arc,
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
        let dispatch_system = Dispatcher::new(None);
        let asset_system = match info.asset_system.take() {
            Some(v) => v,
            None => Default::default(),
        };
        resources.add_resource(asset_system);
        resources.add_resource(dispatch_system);
        resources.add_resource(SceneManager::default());

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

impl<P: PlatformInterface, F: Fn(&mut P, PlatformPreDidInitInput)>
    Into<EngineStateMachine<Initialized>> for (EngineStateMachine<Uninitialized>, &mut P, F)
{
    fn into(self) -> EngineStateMachine<Initialized> {
        let (uninit, interface, init_func) = self;
        let dispatch_system = match uninit.shared.resources.get_resource::<Dispatcher>() {
            Some(v) => Arc::clone(&v),
            None => {
                failure!("Internal engine inconsistency! DispatchSystem should be added to the resource systems!");
            }
        };
        let mut update_thread_local_resources = ThreadLocalResourceManager::default();
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

        let mut builder = MessageBusBuilder::default();
        update_stages.iter_mut().for_each(|stage| {
            stage.register_message_handlers(AnyMessageRegisterer::new(
                &mut builder,
                MessageHandlerType::Update,
            ));
        });
        render_stages.iter_mut().for_each(|stage| {
            stage.register_message_handlers(AnyMessageRegisterer::new(
                &mut builder,
                MessageHandlerType::Render,
            ));
        });
        let mut render_stage_update_handlers = render_stages
            .iter_mut()
            .map(|e| {
                e.create_update_thread_handler(
                    RenderStageUpdateThreadHandlerCreateInfo::new(
                        &mut update_thread_local_resources,
                    ),
                    AnyMessageRegisterer::new(&mut builder, MessageHandlerType::Update),
                )
            })
            .collect::<Vec<_>>();

        uninit.shared.resources.add_resource(builder.build());
        let mut scene_manager = SceneManager::default();

        // Run the platform pre did init function.
        (init_func)(
            interface,
            PlatformPreDidInitInput {
                scene_manager: &mut scene_manager,
                resources: Arc::clone(&uninit.shared.resources),
                update_thread_resources: &mut update_thread_local_resources,
                dispatcher: Arc::clone(&dispatch_system),
            },
        );

        // Run the did init function for all update stages.
        for stage in &mut update_stages {
            match stage.engine_did_initialize(EngineDidInitInput {
                platform_interface: interface,
                scene_manager: &mut scene_manager,
                resources: Arc::clone(&uninit.shared.resources),
                update_thread_resources: &mut update_thread_local_resources,
                dispatcher: Arc::clone(&dispatch_system),
            }) {
                EngineUpdateResult::Ok => continue,
                _ => {
                    failure!("Engine initialization failed.")
                }
            }
        }
        // Run the did init function for all render stages.
        for stage in &mut render_stages {
            match stage.engine_did_initialize(EngineDidInitInput {
                platform_interface: interface,
                scene_manager: &mut scene_manager,
                resources: Arc::clone(&uninit.shared.resources),
                update_thread_resources: &mut update_thread_local_resources,
                dispatcher: Arc::clone(&dispatch_system),
            }) {
                EngineUpdateResult::Ok => continue,
                _ => {
                    failure!("Engine initialization failed.")
                }
            }
        }
        success!("Initialized engine.");
        EngineStateMachine {
            shared: uninit.shared,
            state: Initialized {
                update_stages,
                render_stages,
                scene_manager,
                update_thread_resources: update_thread_local_resources,
                render_stage_update_handlers,
            },
        }
    }
}
