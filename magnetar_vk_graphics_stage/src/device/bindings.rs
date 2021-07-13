use magnetar_engine::PlatformWindowHandle;

use crate::{
    components::Camera, config::VkGraphicsOptions, render_paths::RenderPathDescriptor,
    render_target_bindings::WindowRenderTargetBinding, *,
};

use super::VkInitializedDevice;
use erupt::*;

pub struct CameraRenderPathBinding {
    //TODO: Abstract camera concept later when adding the ECS.
    camera: Camera,
    path: RenderPathDescriptor,
}

pub(crate) struct VkDeviceBindingSet {
    device: VkInitializedDevice,
    camera_bindings: Vec<CameraRenderPathBinding>,
    compatible_paths: Vec<RenderPathDescriptor>,
    window_bindings: Vec<WindowRenderTargetBinding>,
}

impl VkDeviceBindingSet {
    pub fn new(device: VkInitializedDevice, compatible_paths: Vec<RenderPathDescriptor>) -> Self {
        Self {
            device,
            camera_bindings: vec![],
            compatible_paths,
            window_bindings: vec![],
        }
    }

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

    /// Get a reference to the vk device binding set's device.
    pub fn device(&self) -> &VkInitializedDevice {
        &self.device
    }

    /// Get a reference to the vk device binding set's camera bindings.
    pub fn camera_bindings(&self) -> &[CameraRenderPathBinding] {
        self.camera_bindings.as_slice()
    }

    /// Get a reference to the vk device binding set's compatible paths.
    pub fn compatible_paths(&self) -> &[RenderPathDescriptor] {
        self.compatible_paths.as_slice()
    }

    /// Get a reference to the vk device binding set's window bindings.
    pub fn window_bindings(&self) -> &[WindowRenderTargetBinding] {
        self.window_bindings.as_slice()
    }
}
