use egui::*;
use egui_winit::*;
use winit::event::Event;
use winit_platform::*;

mod editor;

pub use editor::*;

fn setup(event: &Event<()>, platform_interface: &mut WinitPlatformInterface) {}

pub fn setup_editor() -> Box<dyn FnMut(&Event<()>, &mut WinitPlatformInterface)> {
    Box::from(setup)
}
