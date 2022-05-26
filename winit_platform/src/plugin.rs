use engine::{engine_stages::*, *};
use winit::{event::*, event_loop::EventLoop, *};

pub trait WinitPlatformPlugin {
    fn pre_run(&mut self, event_loop: EventLoop<()>);
    fn systems_will_init(&mut self, input: PlatformInitInput) -> EngineUpdateResult;
    fn systems_did_init(&mut self, _input: PlatformInitInput) -> EngineUpdateResult;
    fn process_event(&mut self, event: Event<()>);
}
