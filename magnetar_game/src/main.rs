use magnetar_engine::*;
use magnetar_winit_platform::WinitPlatform;

fn main() {
    let create_info = EngineCreateInfo {
        update_tick_rate: 20,
        max_skipped_frames: 0,
        max_frame_rate: Some(60),
        update_stages: vec![],
        render_stages: vec![],
    };
    let engine = Engine::from(create_info);
    let platform = WinitPlatform {};
    engine.run(platform);
}
