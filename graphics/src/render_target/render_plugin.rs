use ash::Instance;
use engine::{PlatformInterface, PlatformWindowHandle};

use crate::{device::GraphicsDevice, Camera, GraphicsOptions, SwapChain, WindowRenderTarget};

use super::AcquiredFrameInfo;

pub struct RenderPluginContext<'a> {
    pub interface: &'a mut dyn PlatformInterface,
    pub window_handle: PlatformWindowHandle,
    pub graphics_device: &'a GraphicsDevice,
    pub options: &'a GraphicsOptions,
    pub camera: &'a Camera,
    pub swap_chain: &'a SwapChain,
}

pub struct RenderPluginRenderInfo<'a> {
    pub info: &'a AcquiredFrameInfo,
    pub context: RenderPluginContext<'a>,
}

pub struct RenderPluginDescriptor {
    create_plugin_fn: fn(
        instance: &Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &mut dyn PlatformInterface,
        window_render_target: &WindowRenderTarget,
        swap_chain: &SwapChain,
        options: &GraphicsOptions,
    ) -> Option<Box<dyn RenderPlugin>>,
}

impl RenderPluginDescriptor {
    pub fn new(
        create_plugin_fn: fn(
            instance: &Instance,
            device: &GraphicsDevice,
            camera: &Camera,
            platform_interface: &mut dyn PlatformInterface,
            window_render_target: &WindowRenderTarget,
            swap_chain: &SwapChain,
            options: &GraphicsOptions,
        ) -> Option<Box<dyn RenderPlugin>>,
    ) -> Self {
        Self { create_plugin_fn }
    }

    /// Get a reference to the render plugin descriptor's create plugin fn.
    pub fn create_plugin_fn(
        &self,
    ) -> fn(
        &Instance,
        &GraphicsDevice,
        &Camera,
        &mut dyn PlatformInterface,
        &WindowRenderTarget,
        &SwapChain,
        &GraphicsOptions,
    ) -> Option<Box<dyn RenderPlugin>> {
        self.create_plugin_fn
    }
}

pub trait RenderPlugin {
    fn create_plugin(
        instance: &Instance,
        device: &GraphicsDevice,
        camera: &Camera,
        platform_interface: &mut dyn PlatformInterface,
        window_render_target: &WindowRenderTarget,
        swap_chain: &SwapChain,
        options: &GraphicsOptions,
    ) -> Option<Box<dyn RenderPlugin>>
    where
        Self: Sized;

    fn swapchain_will_be_resized(&mut self, context: RenderPluginContext<'_>);
    fn swapchain_did_resize(&mut self, context: RenderPluginContext<'_>);
    fn pre_render(&mut self, info: RenderPluginRenderInfo);
    fn post_render(&mut self, info: RenderPluginRenderInfo);
}
