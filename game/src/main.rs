use asset_registry::*;
use engine::engine_stages::RenderStageContainer;
use engine::{engine_stages::*, *};
use graphics::*;
use math::*;
use scripting::*;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::{sync::Arc, vec};
use utils::*;
use winit_platform::WinitPlatform;

mod mesh_writing;

fn create_wasm_scripting_stage<'r>(
    input: UpdateStageConstructorInput<'r>,
) -> Box<dyn AnyUpdateStage> {
    let asset_system: Arc<AssetSystem> = match input.resources.get_resource::<AssetSystem>() {
        Some(v) => v,
        None => {
            fatal!("This system requires an asset system to be present!");
        }
    };
    let mut buffer = vec![];
    let _ = asset_system
        .load_asset_as_blob_into("assets.wasm", "init_test", &mut buffer)
        .unwrap();
    let mut stage = WasmScriptingStage::default();
    stage.add_engine_init_script(&buffer);
    stage.into()
}

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
    let asset_system: Arc<AssetSystem> = match input.resources.get_resource::<AssetSystem>() {
        Some(v) => v,
        None => {
            fatal!("This system requires an asset system to be present!");
        }
    };

    let options = asset_system
        .load_asset_as_type::<GraphicsOptions, _, _>("assets.config", "vulkan")
        .unwrap();

    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("assets.config", "game")
        .unwrap();

    let create_info = GraphicsStageCreateInfo {
        platform: input.platform_interface,
        application_info,
        asset_system,
        options,
    };

    let system = GraphicsStage::new(create_info).expect("Could not initialize render stage.");
    Box::from(RenderStageContainer::from(system))
}

fn main() {
    setup_default_logger();

    mesh_writing::write_meshes();

    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 1,
        max_frame_rate: None,
        update_stages: vec![
            Box::new(create_native_scripting_stage),
            Box::new(create_wasm_scripting_stage),
        ],
        render_stages: vec![Box::new(create_graphics_stage)],
        asset_system: Box::new(|| {
            let asset_system = AssetSystem::default();
            asset_system
                .load_archives_from_directory("./game/asset_archives/", "harchive")
                .unwrap();
            asset_system
        }),
        application_info: Box::new(|_registry| ApplicationInfo::default()),
        concurrency_settings: EngineConcurrencySettings {
            max_async_threads: None,
            max_worker_thread: None,
            fallback_worker_threads: NonZeroUsize::new(8).unwrap(),
            fallback_async_threads: NonZeroUsize::new(2).unwrap(),
        },
        asset_registry: Box::from(|dispatcher| AssetRegistry::new(dispatcher)),
    };
    let engine = Engine::from(create_info);
    let mut platform = WinitPlatform::default();
    engine.run(platform);
}
