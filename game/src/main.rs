use assets::*;
use engine::engine_stages::RenderStageContainer;
use engine::{engine_stages::*, *};
use graphics::*;
use math::*;
use platform_winit::WinitPlatform;
use scripting::*;
use std::num::NonZeroUsize;
use std::sync::Arc;
use utils::dispatcher::Dispatcher;
use utils::*;

pub const IDENTIFIER: &'static str = "GAME";

fn create_native_scripting_stage<'r>(
    _input: UpdateStageConstructorInput<'r>,
) -> Box<dyn AnyUpdateStage> {
    let mut stage = NativeScriptingStage::default();
    stage.add_engine_init_script(|input| {
        match input
            .update_thread_resources
            .get_resource_mut::<CameraManager>()
        {
            Some(manager) => {
                let window = match input.platform_interface.request_window(
                    1024,
                    786,
                    "Title Window",
                    Some("main_window".into()),
                ) {
                    Some(v) => v,
                    None => return EngineUpdateResult::Stop,
                };
                let transform = Transform::new(Vec3f::zero(), Vec4f::zero(), 1.0);
                let camera = manager.create_camera(
                    input.scene_manager.active_scene().handle(),
                    CameraKind::Perspective,
                    RenderPathType::Forward,
                    &transform,
                );

                let registry = input.scene_manager.active_scene_mut().registry_mut();
                let entity = match registry.create_entity((transform, camera)) {
                    Ok(handle) => handle,
                    _ => return EngineUpdateResult::Stop,
                };
                let (camera, transform) = registry
                    .get_components::<(Camera, Transform)>(entity)
                    .unwrap();
                manager.bind_camera_to_window(&transform, &camera, window.handle());
            }
            None => return EngineUpdateResult::Stop,
        };

        EngineUpdateResult::Ok
    });
    stage.add_engine_update_script(|_input| EngineUpdateResult::Ok);
    stage.into()
}

fn create_graphics_stage<'r>(input: RenderStageConstructorInput<'r>) -> Box<dyn AnyRenderStage> {
    let asset_cache: Arc<AssetCache> = match input.resources.get_resource::<AssetCache>() {
        Some(v) => v,
        None => {
            fatal!("This system requires an asset cache to be present!");
        }
    };

    asset_cache.request_asset(asset_id!(assets.config.vulkan));

    let mut buffer = Vec::with_capacity(8192);
    buffer.resize(8192, 0);
    let options: GraphicsOptions = asset_cache
        .load_typed_into(asset_id!(assets.config.vulkan), &mut buffer)
        .unwrap();

    let application_info: ApplicationInfo = asset_cache
        .load_typed_into(asset_id!(assets.config.game), &mut buffer)
        .unwrap();

    let create_info = GraphicsStageCreateInfo {
        platform: input.platform_interface,
        application_info,
        asset_system: asset_cache,
        options,
    };

    let system = GraphicsStage::new(create_info).expect("Could not initialize render stage.");
    Box::from(RenderStageContainer::from(system))
}

fn main() {
    setup_default_logger();
    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 1,
        max_frame_rate: None,
        update_stages: vec![Box::new(create_native_scripting_stage)],
        render_stages: vec![Box::new(create_graphics_stage)],
        application_info: Box::new(|registry| {
            registry.load_typed(asset_id!(assets.config.game)).unwrap()
        }),
        concurrency_settings: EngineConcurrencySettings {
            max_async_threads: None,
            max_worker_thread: None,
            fallback_worker_threads: NonZeroUsize::new(8).unwrap(),
            fallback_async_threads: NonZeroUsize::new(2).unwrap(),
        },
        asset_registry: Box::from(|dispatcher: Arc<Dispatcher>| {
            let registry = AssetRegistry::default();
            dispatcher.spawn_async_blocking(async move {
                let archives = AssetArchive::load_from_directory("./game/asset_archives/", "zarc")
                    .await
                    .expect("Could not load asset archives.");
                archives.into_iter().for_each(|archive| {
                    match registry.register_asset_archive(archive) {
                        Ok(_result) => {
                            t_info!("Loaded archive: {:#?}", _result);
                        }
                        Err((e, _)) => {
                            t_fatal!("Could not load archive {:#?}", e);
                        }
                    }
                });

                registry
            })
        }),
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform::default();
    engine.run(platform);
}
