use std::{cell::RefCell, rc::Rc};

use egui::*;
use egui_winit::*;
use engine::*;
use graphics::*;
use winit_platform::WinitPlatform;

use crate::{EguiIntegration, EguiRenderer};

pub struct EguiRenderPlugin {
    renderer: EguiRenderer,
    context: Context,
    state: Rc<RefCell<(State, Context)>>,
    clipped_primitives: Vec<ClippedPrimitive>,
}

impl RenderPlugin for EguiRenderPlugin {
    fn swapchain_will_be_resized(&mut self, context: RenderPluginContext<'_>) {}

    fn swapchain_did_resize(&mut self, context: RenderPluginContext<'_>) {}

    fn pre_render(&mut self, info: RenderPluginRenderInfo) {
        let platform = info
            .context
            .interface
            .platform_as_any()
            .downcast_mut::<WinitPlatform>()
            .unwrap();
        let window = platform.get_window(info.context.window_handle).unwrap();

        let (state, _) = &mut *self.state.borrow_mut();
        let raw_input = state.take_egui_input(window.window());
        let full_output = self.context.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.add(egui::Label::new("Test Label"));
                if ui.button("Click Me!").clicked() {
                    println!("TESTTESTTES");
                }
            });
        });
        self.clipped_primitives = self.context.tessellate(full_output.shapes);
    }

    fn post_render(&mut self, info: RenderPluginRenderInfo) {
        // TODO: Render the clipped primitives.
    }

    fn create_plugin(
        instance: &ash::Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &mut dyn engine::PlatformInterface,
        window_render_target: &WindowRenderTarget,
        swap_chain: &SwapChain,
        options: &GraphicsOptions,
    ) -> Option<Box<dyn RenderPlugin>>
    where
        Self: Sized,
    {
        let context = Context::default();
        let window = platform_interface.get_window(window_render_target.window())?;
        let egui_winit_state = Rc::new(RefCell::new((
            State::from_pixels_per_point(2048, window.pixels_per_point()),
            context.clone(),
        )));

        let platform = match platform_interface
            .platform_as_any()
            .downcast_mut::<WinitPlatform>()
        {
            Some(v) => v,
            None => return None,
        };

        let integration = match platform.get_plugin_instance::<EguiIntegration>() {
            Some(v) => v,
            None => return None,
        };

        integration.register_plugin(Rc::downgrade(&egui_winit_state));

        let renderer = match EguiRenderer::new(
            instance,
            device,
            camera,
            platform_interface,
            window_render_target,
            swap_chain,
            options,
        ) {
            Some(v) => v,
            None => return None,
        };
        tagged_success!("EGUI", "Set-up the EGUI rendering plugin.");
        Some(Box::new(Self {
            context,
            state: egui_winit_state,
            clipped_primitives: vec![],
            renderer,
        }))
    }
}
