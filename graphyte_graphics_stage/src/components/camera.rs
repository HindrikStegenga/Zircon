use crate::RenderPathType;
use graphyte_engine::ecs::*;
use graphyte_engine::scene_manager::SceneHandle;
use graphyte_math::*;
use graphyte_utils::handles::Handle;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum CameraKind {
    Orthographic = 0,
    Perspective = 1,
}

pub type CameraHandle = Handle<Camera, u16>;

#[derive(Clone, Component)]
pub struct Camera {
    previous_cycle_position: Vec3f,
    previous_cycle_rotation: Vec4f,
    scene: SceneHandle,
    handle: CameraHandle,
    kind: CameraKind,
    path: RenderPathType,
}

impl Camera {
    pub(crate) fn previous_cycle_position(&self) -> Vec3f {
        self.previous_cycle_position
    }
    pub(crate) fn previous_cycle_rotation(&self) -> Vec4f {
        self.previous_cycle_rotation
    }

    pub(crate) fn set_previous_cycle_position(&mut self, position: Vec3f) {
        self.previous_cycle_position = position;
    }

    pub(crate) fn set_previous_cycle_rotation(&mut self, rotation: Vec4f) {
        self.previous_cycle_rotation = rotation;
    }

    pub(crate) fn handle(&self) -> CameraHandle {
        self.handle
    }
    pub fn kind(&self) -> CameraKind {
        self.kind
    }
    pub fn path(&self) -> RenderPathType {
        self.path
    }
}

impl Camera {
    pub(super) fn new(
        scene: SceneHandle,
        handle: CameraHandle,
        kind: CameraKind,
        path: RenderPathType,
        starting_position: Vec3f,
        starting_rotation: Vec4f,
    ) -> Self {
        Camera {
            previous_cycle_position: starting_position,
            previous_cycle_rotation: starting_rotation,
            scene,
            handle,
            kind,
            path,
        }
    }
}
