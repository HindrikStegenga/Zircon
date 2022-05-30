use super::*;
use crate::*;
use ash::*;
use engine::{
    tagged_error, tagged_log, PlatformInterface, PlatformWindowHandle, RenderStageUpdateInput,
};

pub(crate) struct WindowRenderTargetBinding {
    camera: Camera,
    render_path: Box<dyn RenderPath>,
    swap_chain: SwapChain,
    window_render_target: WindowRenderTarget,
    resize_on_sub_optimal: bool,
    render_plugins: Vec<Box<dyn RenderPlugin>>,
}

impl WindowRenderTargetBinding {
    pub fn window_handle(&self) -> PlatformWindowHandle {
        self.window_render_target.window()
    }

    pub fn window_did_resize(
        &mut self,
        device: &GraphicsDevice,
        platform: &dyn PlatformInterface,
        graphics_options: &GraphicsOptions,
    ) -> Result<(), vk::Result> {
        let window = match platform.get_window(self.window_render_target.window()) {
            Some(v) => v,
            None => {
                tagged_error!("Graphics", "Using invalid window handle!");
                return Err(vk::Result::ERROR_INVALID_EXTERNAL_HANDLE_KHR);
            }
        };

        let previous_width = self.swap_chain.current_extent().width;
        let previous_height = self.swap_chain.current_extent().height;

        let width = window.width();
        let height = window.height();

        tagged_log!(
            "Graphics",
            "Previous: {} - {} Current: {} - {}",
            previous_width,
            previous_height,
            width,
            height
        );

        unsafe {
            device.device().device_wait_idle()?;
        }
        self.render_path.swapchain_will_be_resized();
        self.swap_chain.resize_swap_chain(
            window,
            &self.window_render_target,
            width,
            height,
            graphics_options,
            device,
        )?;
        self.render_path.swapchain_did_resize(
            &mut self.camera,
            &mut self.swap_chain,
            &mut self.window_render_target,
            device,
        );
        Ok(())
    }

    pub fn render(
        &mut self,
        device: &GraphicsDevice,
        input: &mut RenderStageUpdateInput,
        graphics_options: &GraphicsOptions,
    ) -> bool {
        if self.swap_chain.current_extent().width == 0
            && self.swap_chain.current_extent().height == 0
        {
            return true;
        }

        // Acquire the swap chain image to render into
        let (info, is_sub_optimal) = match unsafe { self.swap_chain.acquire_next_frame() } {
            Ok(value) => value,
            Err(e) => match e {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    if !self
                        .window_did_resize(device, input.platform, graphics_options)
                        .is_ok()
                    {
                        return false;
                    };
                    return true;
                }
                e => {
                    tagged_error!("Graphics", "Error during acquiring of next frame: {}", e);
                    return false;
                }
            },
        };
        if is_sub_optimal && self.resize_on_sub_optimal {
            if !self
                .window_did_resize(device, input.platform, graphics_options)
                .is_ok()
            {
                return false;
            };
            return true;
        }

        let window_handle = self.window_handle();
        // Pre-render for the render plugins
        for plugin in &mut self.render_plugins {
            plugin.pre_render(RenderPluginRenderInfo {
                info: &info,
                context: RenderPluginContext {
                    interface: input.platform,
                    graphics_device: device,
                    options: graphics_options,
                    camera: &self.camera,
                    swap_chain: &self.swap_chain,
                    window_handle,
                },
            })
        }

        // Render
        self.render_path.render(
            &self.camera,
            &info,
            &mut self.window_render_target,
            device,
            input,
        );

        // Post-render for the render plugins
        for plugin in &mut self.render_plugins {
            plugin.post_render(RenderPluginRenderInfo {
                info: &info,
                context: RenderPluginContext {
                    interface: input.platform,
                    graphics_device: device,
                    options: graphics_options,
                    camera: &self.camera,
                    swap_chain: &self.swap_chain,
                    window_handle,
                },
            })
        }

        // Present the frame to the screen
        match unsafe {
            self.swap_chain
                .present_frame(info.image_index, &device.graphics_queue())
        } {
            Ok(is_sub_optimal) => {
                if is_sub_optimal && self.resize_on_sub_optimal {
                    if !self
                        .window_did_resize(device, input.platform, graphics_options)
                        .is_ok()
                    {
                        return false;
                    };
                    return true;
                }
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                if !self
                    .window_did_resize(device, input.platform, graphics_options)
                    .is_ok()
                {
                    return false;
                };
                return true;
            }
            Err(e) => {
                tagged_error!("Graphics", "Presentation error: {}", e);
                return false;
            }
        };
        true
    }
}

impl WindowRenderTargetBinding {
    pub fn new(
        instance: &Instance,
        graphics_device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &mut dyn PlatformInterface,
        mut window_render_target: WindowRenderTarget,
        plugin_descriptors: &[RenderPluginDescriptor],
        options: &GraphicsOptions,
    ) -> Result<Self, WindowRenderTarget> {
        // Get the window
        let window = match platform_interface.get_window(window_render_target.window()) {
            Some(v) => v,
            None => return Err(window_render_target),
        };
        // Get the swap chain
        let mut swap_chain = match SwapChain::new(
            instance,
            graphics_device,
            window,
            &window_render_target,
            options,
        ) {
            Some(v) => v,
            None => return Err(window_render_target),
        };
        // Create a render path
        let render_path = match camera.path() {
            RenderPathType::Forward => {
                match ForwardRenderPath::new(RenderPathCreateInfo {
                    options,
                    graphics_device,
                    camera,
                    swap_chain: &mut swap_chain,
                    window_render_target: &mut window_render_target,
                }) {
                    Some(v) => v,
                    None => return Err(window_render_target),
                }
            }
        };

        let render_plugins = plugin_descriptors
            .into_iter()
            .filter_map(|e| {
                (e.create_plugin_fn())(
                    instance,
                    graphics_device,
                    camera,
                    platform_interface,
                    &window_render_target,
                    &swap_chain,
                    options,
                )
            })
            .collect();

        Ok(WindowRenderTargetBinding {
            window_render_target,
            camera: camera.clone(),
            swap_chain,
            render_path: Box::new(render_path),
            resize_on_sub_optimal: options.resize_on_sub_optimal,
            render_plugins,
        })
    }
}
