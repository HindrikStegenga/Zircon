use std::{sync::Arc, vec};

use graphyte_engine::engine_stages::RenderStageContainer;
use graphyte_engine::{engine::create_info::ApplicationInfo, engine_stages::*, *};
use graphyte_graphics_stage::{GraphicsStage, GraphicsStageCreateInfo};

use graphyte_graphics_stage::vulkan::*;

use graphyte_winit_platform::WinitPlatform;

struct TestStage {}
impl UpdateStage for TestStage {
    const IDENTIFIER: &'static str = "TestStage";

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        input.dispatcher().dispatch_async(async {
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
        });
        EngineUpdateResult::Ok
    }
}

fn create_graphics_stage<'r>(input: RenderStageConstructorInput<'r>) -> Box<dyn AnyRenderStage> {
    let asset_system: Arc<AssetSystem> = match input.resources().get_engine_resource::<AssetSystem>()
    {
        Some(v) => v,
        None => {
            failure!("This system requires an asset system to be present!")
        }
    };

    let vulkan_graphics_options: VkGraphicsOptions = asset_system
        .load_asset_as_type::<VkGraphicsOptions, _, _>("config", "vulkan")
        .unwrap();

    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("config", "game")
        .unwrap();

    let create_info = GraphicsStageCreateInfo {
        preferred_api: "vulkan".to_string(),
        application_info,
        platform: input.platform_interface,
        asset_system,
        //#[cfg(feature = "vulkan_api")]
        vulkan: vulkan_graphics_options,
        //#[cfg(feature = "open_gl_api")]
        open_gl: (),
        //#[cfg(feature = "metal_api")]
        //metal: ()
    };

    let system = GraphicsStage::new(create_info);
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
        max_frame_rate: Some(60),
        update_stages: vec![Box::new(|_input: UpdateStageConstructorInput<'_>| {
            TestStage {}.into()
        })],
        render_stages: vec![Box::new(create_graphics_stage)],
        asset_system: Some(asset_system),
        application_info,
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform::default();
    engine.run(platform);
}
