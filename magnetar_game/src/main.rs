use std::vec;

use magnetar_engine::{engine_stages::*, *};
use magnetar_winit_platform::WinitPlatform;

struct TestStage {}
impl UpdateStage for TestStage {
    const IDENTIFIER: &'static str = "TestStage";

    fn update(&mut self, input: &mut UpdateStageUpdateInput) -> EngineUpdateResult {
        input.dispatcher().dispatch_async(async {
            println!("Running async!");
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
            println!("Still running async!");
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
            println!("Finishing async!");
        });
        EngineUpdateResult::Ok
    }
}

fn test<'r, 's>(
    _input: &'r mut UpdateStageConstructorInput<'s>,
) -> Box<dyn AnyUpdateStage + 'static> {
    Box::new(TestStage {})
}

fn main() {
    let create_info = EngineCreateInfo {
        update_tick_rate: 1,
        max_skipped_frames: 0,
        max_frame_rate: Some(60),
        update_stages: vec![Box::from(test)],
        render_stages: vec![],
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform {};
    engine.run(platform);
}
