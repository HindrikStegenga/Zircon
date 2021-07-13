use magnetar_engine::PlatformWindowHandle;

use crate::{
    components::Camera,
    config::VkGraphicsOptions,
    render_paths::{ForwardRenderPath, RenderPathDescriptor},
    render_target_bindings::WindowRenderTargetBinding,
    *,
};

use super::VkInitializedDevice;
use erupt::*;

pub(crate) struct CameraRenderPathBinding {
    //TODO: Abstract camera concept later when adding the ECS.
    camera: Camera,
    path: RenderPathInstance,
    window_binding: WindowRenderTargetBinding,
}

pub(crate) enum RenderPathInstance {
    Forward(ForwardRenderPath),
    Deferred(),
}

impl RenderPathInstance {
    pub fn descriptor(&self) -> RenderPathDescriptor {
        match self {
            RenderPathInstance::Forward(_) => RenderPathDescriptor::new::<ForwardRenderPath>(),
            RenderPathInstance::Deferred() => todo!(),
        }
    }
}

pub(crate) struct VkDeviceBindingSet {
    device: VkInitializedDevice,
    bindings: Vec<CameraRenderPathBinding>,
    compatible_paths: Vec<RenderPathDescriptor>,
    available_window_bindings: Vec<WindowRenderTargetBinding>,
}

impl VkDeviceBindingSet {
    pub fn new(device: VkInitializedDevice, compatible_paths: Vec<RenderPathDescriptor>) -> Self {
        assert!(!compatible_paths.is_empty());
        Self {
            device,
            bindings: vec![],
            compatible_paths,
            available_window_bindings: vec![],
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
        self.available_window_bindings.push(v.unwrap());
    }

    pub fn bind_camera_to_first_available_binding(&mut self, camera: Camera) -> bool {
        if self.available_window_bindings().is_empty() {
            return false;
        }
        let path = if let Some(v) = self
            .compatible_paths()
            .iter()
            .find(|e| &e.render_path_type() == camera.preferred_render_path())
        {
            v
        } else {
            self.compatible_paths().first().unwrap()
        };
        let instance = path.create_instance();
        let binding = CameraRenderPathBinding {
            camera,
            path: instance,
            window_binding: self.available_window_bindings.pop().unwrap(),
        };
        self.bindings.push(binding);

        true
    }

    /// Get a reference to the vk device binding set's device.
    pub fn device(&self) -> &VkInitializedDevice {
        &self.device
    }

    /// Get a reference to the vk device binding set's camera bindings.
    pub fn camera_bindings(&self) -> &[CameraRenderPathBinding] {
        self.bindings.as_slice()
    }

    /// Get a reference to the vk device binding set's compatible paths.
    pub fn compatible_paths(&self) -> &[RenderPathDescriptor] {
        self.compatible_paths.as_slice()
    }

    /// Get a reference to the vk device binding set's available window bindings.
    pub fn available_window_bindings(&self) -> &[WindowRenderTargetBinding] {
        self.available_window_bindings.as_slice()
    }

    /// Get a reference to the vk device binding set's bindings.
    pub(crate) fn bindings(&self) -> &[CameraRenderPathBinding] {
        self.bindings.as_slice()
    }
}
