use egui_winit_ash_integration::*;
use winit::event::Event;
use winit_platform::*;

fn setup(event: &Event<()>, platform_interface: &mut WinitPlatformInterface) {}

pub fn setup_editor() -> Box<dyn FnMut(&Event<()>, &mut WinitPlatformInterface)> {
    Box::from(setup)
}
