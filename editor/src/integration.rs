use std::{cell::RefCell, rc::Weak};
use utils::*;
use engine::*;
use graphics::*;
use winit_platform::{plugin::WinitPlatformPlugin, WinitPlatform, WinitPlatformInterface};

pub struct EguiIntegration {
    active_plugin_states: Vec<Weak<RefCell<(egui_winit::State, egui::Context)>>>,
}

impl EguiIntegration {
    pub fn new() -> Self {
        Self {
            active_plugin_states: vec![],
        }
    }

    pub fn register_plugin(&mut self, state: Weak<RefCell<(egui_winit::State, egui::Context)>>) {
        self.active_plugin_states.push(state)
    }
}

impl WinitPlatformPlugin for EguiIntegration {
    fn pre_run(&mut self, event_loop: winit::event_loop::EventLoop<()>) {}

    fn systems_will_init(
        &mut self,
        interface: &mut WinitPlatformInterface,
        input: &mut engine::engine_stages::PlatformInitInput,
    ) -> engine::EngineUpdateResult {
        engine::EngineUpdateResult::Ok
    }

    fn systems_did_init(
        &mut self,
        interface: &mut WinitPlatformInterface,
        input: &mut engine::engine_stages::PlatformInitInput,
    ) -> engine::EngineUpdateResult {
        let _ = match interface.platform_as_any().downcast_mut::<WinitPlatform>() {
            Some(v) => v,
            None => {
                t_error!(
                    "This integration requires to be run on the `WinitPlatform` platform."
                );
                return EngineUpdateResult::Stop;
            }
        };

        let render_system = match input.render_stage_manager.get_stage::<GraphicsStage>() {
            Some(v) => v,
            None => {
                t_error!(
                    "This integration requires the stage `GraphicsStage` to be present."
                );
                return engine::EngineUpdateResult::Stop;
            }
        };
        render_system.add_render_plugin::<crate::EguiRenderPlugin>();
        engine::EngineUpdateResult::Ok
    }

    // Returns true if event is captured.
    fn process_event(&mut self, event: &winit::event::Event<()>) -> bool {
        let mut is_captured = false;
        match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event,
            } => {
                for i in (0..self.active_plugin_states.len()).rev() {
                    let state = &mut self.active_plugin_states[i];
                    if let Some(state) = state.upgrade() {
                        let (state, ctx) = &mut *state.borrow_mut();
                        if state.on_event(ctx, event) {
                            is_captured = true;
                        };
                    } else {
                        self.active_plugin_states.remove(i);
                    }
                }
            }
            _ => (),
        }
        is_captured
    }
}
