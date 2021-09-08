use std::{sync::Arc, vec};

use spacebar_engine::{engine::create_info::ApplicationInfo, engine_stages::*, *};
use spacebar_vk_graphics_stage::{config::VkGraphicsOptions, *};
use spacebar_winit_platform::WinitPlatform;

struct TestStage {}
impl UpdateStage for TestStage {
    const IDENTIFIER: &'static str = "TestStage";

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        input.dispatcher().dispatch_async(async {
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
        });
        EngineUpdateResult::Ok
    }
}

fn creage_graphics_stage<'r>(input: RenderStageConstructorInput<'r>) -> Box<dyn AnyRenderStage> {
    let asset_system: &mut Arc<AssetSystem> = match input
        .render_thread_resources
        .get_unique_resource_mut::<Arc<AssetSystem>>()
    {
        Some(v) => v,
        None => {
            failure!("This system requires an asset system to be present!")
        }
    };
    let asset_system = Arc::clone(asset_system);

    let graphics_options: VkGraphicsOptions = asset_system
        .load_asset_as_type::<VkGraphicsOptions, _, _>("config", "vulkan")
        .unwrap();

    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("config", "game")
        .unwrap();

    let create_info = VkGraphicsSystemCreateInfo {
        graphics_options,
        application_info,
        render_thread_resources: input.render_thread_resources,
        platform_interface: input.platform_interface,
        asset_system,
    };

    let system = VkGraphicsStage::new(create_info).expect("Could not create VkGraphicsSystem!");
    Box::new(system)
}

fn main() {
    let asset_system = Arc::new(AssetSystem::default());
    asset_system
        .load_files_from_directory("./magnetar_game/asset_archives/config", "config")
        .unwrap();
    let application_info = asset_system
        .load_asset_as_type::<ApplicationInfo, _, _>("config", "game")
        .unwrap();

    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 1,
        max_frame_rate: None,
        update_stages: vec![Box::new(|input: UpdateStageConstructorInput<'_>| {
            let asset_system: &mut Arc<AssetSystem> = match input
                .update_thread_resources
                .get_unique_resource_mut::<Arc<AssetSystem>>()
            {
                Some(v) => v,
                None => {
                    failure!("This system requires an asset system to be present!")
                }
            };
            Box::new(TestStage {})
        })],
        render_stages: vec![Box::new(creage_graphics_stage)],
        asset_system: Some(asset_system),
        application_info,
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform::default();
    engine.run(platform);
}
