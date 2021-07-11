use magnetar_engine::PlatformWindowHandle;

use crate::{
    config::VkGraphicsOptions, render_paths::RenderPathDescriptor,
    render_target_bindings::WindowRenderTargetBinding, *,
};

use super::VkInitializedDevice;
use erupt::*;
pub struct VkDeviceBindingSet {
    pub(crate) device: VkInitializedDevice,
    pub(crate) paths: Vec<RenderPathDescriptor>,
    pub(crate) window_bindings: Vec<WindowRenderTargetBinding>,
}

impl VkDeviceBindingSet {
    pub fn add_window_render_target_binding(
        &mut self,
        instance: VkInstance,
        graphics_options: &VkGraphicsOptions,
        window_handle: PlatformWindowHandle,
        surface: vk::SurfaceKHR,
    ) {
        let v = WindowRenderTargetBinding::new(
            instance,
            graphics_options,
            &self.device,
            window_handle,
            surface,
        );
        self.window_bindings.push(v.unwrap());
    }
}
