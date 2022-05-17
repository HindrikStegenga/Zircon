use super::*;
use crate::*;
use ash::Instance;
use graphyte_engine::PlatformInterface;

pub(crate) struct WindowRenderTargetBinding {
    camera: Camera,
    render_path: Box<dyn RenderPath>,
    swap_chain: SwapChain,
    window_render_target: WindowRenderTarget,
}

impl WindowRenderTargetBinding {
    pub fn window_render_target(&self) -> &WindowRenderTarget {
        &self.window_render_target
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn swap_chain(&self) -> &SwapChain {
        &self.swap_chain
    }
    pub fn render_path(&self) -> &Box<dyn RenderPath> {
        &self.render_path
    }

    pub fn render(&mut self, device: &GraphicsDevice) -> bool {
        self.render_path.render(
            &self.camera,
            &mut self.swap_chain,
            &mut self.window_render_target,
            device,
        )
    }
}

impl WindowRenderTargetBinding {
    pub fn new(
        instance: &Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &dyn PlatformInterface,
        window_render_target: WindowRenderTarget,
        options: &GraphicsOptions,
    ) -> Result<Self, WindowRenderTarget> {
        // Get the window
        let window = match platform_interface.get_window(window_render_target.window()) {
            Some(v) => v,
            None => return Err(window_render_target),
        };
        // Get the swap chain
        let swap_chain =
            match SwapChain::new(instance, device, window, &window_render_target, options) {
                Some(v) => v,
                None => return Err(window_render_target),
            };
        // Create a render path
        let render_path = match camera.path() {
            RenderPathType::Forward => {
                match ForwardRenderPath::instantiate(RenderPathCreateInfo { options }) {
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
