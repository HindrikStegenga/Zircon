use std::vec;

use magnetar_engine::{engine_stages::*, *};
use magnetar_winit_platform::WinitPlatform;

struct TestStage {}
impl UpdateStage for TestStage {
    const IDENTIFIER: &'static str = "TestStage";

    fn update(&mut self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult {
        input.dispatcher().dispatch_async(async {
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
        });
        EngineUpdateResult::Ok
    }
}

fn main() {
    let create_info = EngineCreateInfo {
        update_tick_rate: 1,
        max_skipped_frames: 0,
        max_frame_rate: Some(60),
        update_stages: vec![Box::new(|input| {
            input
                .resources
                .asset_system
                .load_archives_from_directory("./tmp/", "mtra")
                .unwrap();
            input
                .resources
                .asset_system
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
