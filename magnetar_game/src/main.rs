use std::{sync::Arc, vec};

use magnetar_engine::{engine_stages::*, *};
use magnetar_winit_platform::WinitPlatform;

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

fn main() {
    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 1,
        max_frame_rate: None,
        update_stages: vec![Box::new(|input: UpdateStageConstructorInput<'_>| {
            let asset_system: &mut Arc<AssetSystem> = match input
                .resource_system
                .get_unique_resource_mut::<Arc<AssetSystem>>()
            {
                Some(v) => v,
                None => {
                    failure!("This system requires an asset system to be present!")
                }
            };

            asset_system
                .load_archives_from_directory("./tmp/", "mtra")
                .unwrap();
            asset_system
                .load_files_from_directory("./magnetar_game/asset_archives/config", "config")
                .unwrap();
            Box::new(TestStage {})
        })],
        render_stages: vec![],
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform::default();
    engine.run(platform);
}
