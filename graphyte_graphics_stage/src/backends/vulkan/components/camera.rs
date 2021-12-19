use graphyte_engine::PlatformWindowHandle;

use crate::vulkan::render_paths::*;

pub enum CameraType {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera),
}

pub struct OrthographicCamera {}
pub struct PerspectiveCamera {}

pub enum CameraTargetBinding {
    Window(PlatformWindowHandle),
    Texture,
}

pub struct Camera {
    camera_type: CameraType,
    binding: CameraTargetBinding,
    preferred_render_path: RenderPathType,
}

impl Camera {
    pub fn new(
        camera_type: CameraType,
        binding: CameraTargetBinding,
        preferred_render_path: RenderPathType,
    ) -> Self {
        Self {
            camera_type,
            binding,
            preferred_render_path,
        }
    }

    /// Get a reference to the camera's camera type.
    pub fn camera_type(&self) -> &CameraType {
        &self.camera_type
    }

    /// Get a reference to the camera's binding.
    pub fn binding(&self) -> &CameraTargetBinding {
        &self.binding
    }

    /// Get a reference to the camera's preferred render path.
    pub fn preferred_render_path(&self) -> &RenderPathType {
        &self.preferred_render_path
    }
}
