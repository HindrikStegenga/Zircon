use super::*;
use crate::*;
use ash::Instance;
use graphyte_engine::{
    tagged_error, PlatformInterface, PlatformWindowHandle, RenderStageUpdateInput,
};

pub(crate) struct WindowRenderTargetBinding {
    camera: Camera,
    render_path: Box<dyn RenderPath>,
    swap_chain: SwapChain,
    window_render_target: WindowRenderTarget,
}

impl WindowRenderTargetBinding {
    pub fn window_handle(&self) -> PlatformWindowHandle {
        self.window_render_target.window()
    }

    pub fn window_did_resize(&mut self, device: &GraphicsDevice, platform: &dyn PlatformInterface) {
        let window = match platform.get_window(self.window_render_target.window()) {
            Some(v) => v,
            None => {
                tagged_error!("Graphics", "Using invalid window handle!");
                return;
            }
        };
        let width = window.width();
        let height = window.height();
    }

    pub fn render(&mut self, device: &GraphicsDevice, input: &mut RenderStageUpdateInput) -> bool {
        self.render_path.render(
            &self.camera,
            &mut self.swap_chain,
            &mut self.window_render_target,
            device,
            input,
        )
    }
}

impl WindowRenderTargetBinding {
    pub fn new(
        instance: &Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &dyn PlatformInterface,
        mut window_render_target: WindowRenderTarget,
        options: &GraphicsOptions,
    ) -> Result<Self, WindowRenderTarget> {
        // Get the window
        let window = match platform_interface.get_window(window_render_target.window()) {
            Some(v) => v,
            None => return Err(window_render_target),
        };
        // Get the swap chain
        let mut swap_chain =
            match SwapChain::new(instance, device, window, &window_render_target, options) {
                Some(v) => v,
                None => return Err(window_render_target),
            };
        // Create a render path
        let render_path = match camera.path() {
            RenderPathType::Forward => {
                match ForwardRenderPath::instantiate(RenderPathCreateInfo {
                    options,
                    graphics_device: device,
                    camera,
                    swap_chain: &mut swap_chain,
                    window_render_target: &mut window_render_target,
                }) {
                    Some(v) => v,
                    None => return Err(window_render_target),
                }
            }
        };

        Ok(WindowRenderTargetBinding {
            window_render_target,
            camera: camera.clone(),
            swap_chain,
            render_path: Box::new(render_path),
        })
    }
}
