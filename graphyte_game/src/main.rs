use std::{sync::Arc, vec};

use graphyte_engine::engine_stages::RenderStageContainer;
use graphyte_engine::{engine::create_info::ApplicationInfo, engine_stages::*, *};
use graphyte_graphics_stage::*;
use graphyte_math::*;
use graphyte_scripting_stage::NativeScriptingStage;
use graphyte_winit_platform::WinitPlatform;

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

                let mut registry = input.scene_manager.active_scene_mut().registry_mut();
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
    stage.into()
}

fn create_graphics_stage<'r>(input: RenderStageConstructorInput<'r>) -> Box<dyn AnyRenderStage> {
    let asset_system: Arc<AssetSystem> = match input.resources.get_resource::<AssetSystem>() {
        Some(v) => v,
        None => {
            failure!("This system requires an asset system to be present!")
        }
    };

    let options = asset_system
        .load_asset_as_type::<GraphicsOptions, _, _>("config", "vulkan")
        .unwrap();

    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("config", "game")
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
    let asset_system = AssetSystem::default();
    asset_system
        .load_files_from_directory("./graphyte_game/asset_archives/config", "config")
        .unwrap();
    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("config", "game")
        .unwrap();

    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 1,
        max_frame_rate: None,
        update_stages: vec![Box::new(create_native_scripting_stage)],
        render_stages: vec![Box::new(create_graphics_stage)],
        asset_system: Some(asset_system),
        application_info,
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform::default();
    engine.run(platform);
}
