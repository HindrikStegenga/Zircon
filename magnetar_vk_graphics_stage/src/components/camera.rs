use magnetar_engine::PlatformWindowHandle;

use crate::render_paths::RenderPathType;

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
    render_path: RenderPathType,
}

impl Camera {
    pub fn new(camera_type: CameraType, binding: CameraTargetBinding) -> Self {
        Self {
            camera_type,
            binding,
            render_path: (),
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
}
