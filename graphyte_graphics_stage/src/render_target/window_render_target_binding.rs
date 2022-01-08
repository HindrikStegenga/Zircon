use super::*;
use crate::*;
use ash::Instance;
use graphyte_engine::PlatformInterface;

pub(crate) struct WindowRenderTargetBinding {
    window_render_target: WindowRenderTarget,
    swap_chain: SwapChain,
    render_path: Box<dyn RenderPath>,
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

        Err(window_render_target)
    }
}
