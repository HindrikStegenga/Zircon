use super::*;
use crate::message_bus::{AnyMessageRegisterer, MessageBusBuilder, MessageHandlerType};
use crate::scene_manager::SceneManager;
use crate::{engine::gameloop_timer::*, engine_stages::*, resource_manager::*, *};
use asset_library::split_view::SplitViewMut;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use utils::dispatcher::Dispatcher;

pub struct Uninitialized {}

impl EngineStateMachine<Uninitialized> {
    pub fn new(info: EngineCreateInfo) -> Self {
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
        let asset_system = match info.asset_system {
            Some(ref v) => (v)(),
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

impl<P: PlatformInterface + PlatformInitalizationHandler> Into<EngineStateMachine<Initialized>>
    for (EngineStateMachine<Uninitialized>, &mut P)
{
    fn into(self) -> EngineStateMachine<Initialized> {
        let (uninit, interface) = self;
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
        let render_stage_update_handlers = render_stages
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
        match interface.systems_will_init(PlatformInitInput {
            scene_manager: &mut scene_manager,
            resources: Arc::clone(&uninit.shared.resources),
            update_thread_resources: &mut update_thread_local_resources,
            dispatcher: Arc::clone(&dispatch_system),
            update_stage_manager: UpdateStageManager::from_slice(&mut update_stages),
            render_stage_manager: RenderStageManager::from_slice(&mut render_stages),
        }) {
            EngineUpdateResult::Ok => (),
            e => tagged_failure!("Engine", "Engine initialization failed: {:#?}", e),
        }

        // Run the did init function for all update stages.
        if let Err(e) = SplitViewMut::for_each_until_error(&mut update_stages, |mut split_view| {
            let (before, stage, after) = split_view.components_mut();
            let update_stage_manager = UpdateStageManager::from_slices(before, after);
            let render_stage_manager = RenderStageManager::from_slice(&mut render_stages);
            match stage.engine_did_initialize(EngineDidInitInput {
                platform_interface: interface,
                scene_manager: &mut scene_manager,
                resources: Arc::clone(&uninit.shared.resources),
                update_thread_resources: &mut update_thread_local_resources,
                dispatcher: Arc::clone(&dispatch_system),
                update_stage_manager,
                render_stage_manager,
            }) {
                EngineUpdateResult::Ok => Ok(()),
                value => Err(value),
            }
        }) {
            tagged_failure!("Engine", "Engine initialization failed: {:#?}", e);
        }

        // Run the did init function for all render stages.
        if let Err(e) = SplitViewMut::for_each_until_error(&mut render_stages, |mut split_view| {
            let (before, stage, after) = split_view.components_mut();
            let update_stage_manager = UpdateStageManager::from_slice(&mut update_stages);
            let render_stage_manager = RenderStageManager::from_slices(before, after);

            match stage.engine_did_initialize(EngineDidInitInput {
                platform_interface: interface,
                scene_manager: &mut scene_manager,
                resources: Arc::clone(&uninit.shared.resources),
                update_thread_resources: &mut update_thread_local_resources,
                dispatcher: Arc::clone(&dispatch_system),
                update_stage_manager,
                render_stage_manager,
            }) {
                EngineUpdateResult::Ok => Ok(()),
                value => Err(value),
            }
        }) {
            tagged_failure!("Engine", "Engine initialization failed: {:#?}", e);
        };

        // Run the platform post did init function.
        match interface.systems_did_init(PlatformInitInput {
            scene_manager: &mut scene_manager,
            resources: Arc::clone(&uninit.shared.resources),
            update_thread_resources: &mut update_thread_local_resources,
            dispatcher: Arc::clone(&dispatch_system),
            update_stage_manager: UpdateStageManager::from_slice(&mut update_stages),
            render_stage_manager: RenderStageManager::from_slice(&mut render_stages),
        }) {
            EngineUpdateResult::Ok => (),
            e => tagged_failure!("Engine", "Engine initialization failed: {:#?}", e),
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
