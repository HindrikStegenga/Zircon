use graphyte_engine::PlatformWindowHandle;
use crate::{Camera, CameraHandle, CameraKind, RenderPathType, Transform};
use graphyte_engine::scene_manager::SceneHandle;
use graphyte_math::{Vec3f, Vec4f};

pub struct CameraManager {
    handle_counter: u16,
}

impl CameraManager {

    pub fn new() -> CameraManager {
        Self {
            handle_counter: 0
        }
    }

    pub fn create_camera(
        &mut self,
        scene: SceneHandle,
        kind: CameraKind,
        path: RenderPathType,
    ) -> Camera {
        let id: u16 = self.handle_counter;
        let camera = Camera::new(scene, CameraHandle::from(id), kind, path);
        self.handle_counter += 1;
        camera
    }
}

#[derive(Clone)]
pub(crate) struct CameraState {
    previous_cycle_position: Vec3f,
    previous_cycle_rotation: Vec4f,
    current_cycle_position: Vec3f,
    current_cycle_rotation: Vec4f,
}

#[derive(Clone)]
pub(crate) struct CameraStatesUpdate {
    cameras: Vec<CameraState>,
}

#[derive(Clone)]
pub(crate) struct CameraIsBoundToWindow {
    position: Vec3f,
    rotation: Vec4f,
    camera: Camera,
    window_handle: PlatformWindowHandle
}