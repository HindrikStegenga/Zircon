use crate::{Camera, CameraHandle, CameraKind, RenderPathType};
use graphyte_engine::scene_manager::SceneHandle;

pub struct CameraManager {
    handle_counter: u16,
}

impl CameraManager {
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
pub(crate) struct CameraStatesUpdate {
    cameras: Vec<Camera>,
}
