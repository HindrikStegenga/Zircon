use crate::{Camera, CameraHandle, CameraKind, RenderPathType, Transform};
use crossbeam::channel::*;
use engine::ecs::Registry;
use engine::scene_manager::SceneHandle;
use engine::PlatformWindowHandle;
use math::{Vec3f, Vec4f};
use utils::tagged_warn;

pub struct CameraManager {
    handle_counter: u16,
    cameras_updated_sender: Sender<Vec<CameraStateUpdate>>,
    camera_is_bound_sender: Sender<CameraIsBoundToWindow>,
    camera_is_unbound_sender: Sender<CameraIsUnbound>,
}

impl CameraManager {
    pub(crate) fn new(
        cameras_updated_sender: Sender<Vec<CameraStateUpdate>>,
        camera_is_bound_sender: Sender<CameraIsBoundToWindow>,
        camera_is_unbound_sender: Sender<CameraIsUnbound>,
    ) -> CameraManager {
        Self {
            handle_counter: 0,
            cameras_updated_sender,
            camera_is_bound_sender,
            camera_is_unbound_sender,
        }
    }

    /// Creates a new camera. Camera's must only be added to entities containing a Transform component!!
    /// Otherwise, they will be ignored when updating the camera state every update cycle.
    pub fn create_camera(
        &mut self,
        scene: SceneHandle,
        kind: CameraKind,
        path: RenderPathType,
        transform: &Transform,
    ) -> Camera {
        let id: u16 = self.handle_counter;
        let camera = Camera::new(
            scene,
            CameraHandle::from(id),
            kind,
            path,
            transform.position(),
            transform.rotation(),
        );
        self.handle_counter += 1;
        camera
    }

    /// Binding a camera implicitly unbinds the previous camera.
    pub fn bind_camera_to_window(
        &mut self,
        transform: &Transform,
        camera: &Camera,
        window_handle: PlatformWindowHandle,
    ) {
        self.camera_is_bound_sender.send(CameraIsBoundToWindow {
            position: transform.position(),
            rotation: transform.rotation(),
            camera: camera.clone(),
            window_handle,
        });
    }

    /// Explicitly unbinds a camera.
    pub fn unbind_camera(&mut self, camera: &Camera) {
        self.camera_is_unbound_sender.send(CameraIsUnbound {
            camera: camera.handle(),
        });
    }

    /// Must not be called externally. Internal function which updates the camera state on the main thread for rendering.
    pub(crate) fn update_cameras(&mut self, registry: &mut Registry) {
        let mut camera_updates: Vec<CameraStateUpdate> = Vec::with_capacity(16);
        for (transforms, cameras) in registry.iter_components_matching_mut::<(Transform, Camera)>()
        {
            for (transform, camera) in transforms.iter_mut().zip(cameras) {
                let previous_position = camera.previous_cycle_position();
                let previous_rotation = camera.previous_cycle_rotation();
                camera_updates.push(CameraStateUpdate {
                    previous_cycle_position: previous_position,
                    previous_cycle_rotation: previous_rotation,
                    current_cycle_position: transform.position(),
                    current_cycle_rotation: transform.rotation(),
                    camera_handle: camera.handle(),
                });
                camera.set_previous_cycle_position(transform.position());
                camera.set_previous_cycle_rotation(transform.rotation());
            }
        }
        if let Err(e) = self.cameras_updated_sender.send(camera_updates) {
            tagged_warn!("Graphics", "Camera update failed!");
        }
    }
}

#[derive(Clone)]
pub(crate) struct CameraState {
    pub(crate) previous_cycle_position: Vec3f,
    pub(crate) previous_cycle_rotation: Vec4f,
    pub(crate) camera: Camera,
}

#[derive(Clone)]
pub(crate) struct CameraStateUpdate {
    pub(crate) previous_cycle_position: Vec3f,
    pub(crate) previous_cycle_rotation: Vec4f,
    pub(crate) current_cycle_position: Vec3f,
    pub(crate) current_cycle_rotation: Vec4f,
    pub(crate) camera_handle: CameraHandle,
}

#[derive(Clone)]
pub(crate) struct CameraIsBoundToWindow {
    pub(crate) position: Vec3f,
    pub(crate) rotation: Vec4f,
    pub(crate) camera: Camera,
    pub(crate) window_handle: PlatformWindowHandle,
}

#[derive(Clone)]
pub(crate) struct CameraIsUnbound {
    pub(crate) camera: CameraHandle,
}
