use crate::RenderPathType;
use graphyte_engine::ecs::*;
use graphyte_engine::scene_manager::SceneHandle;
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
    scene: SceneHandle,
    handle: CameraHandle,
    kind: CameraKind,
    path: RenderPathType,
}

impl Camera {
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
    ) -> Self {
        Camera {
            scene,
            handle,
            kind,
            path,
        }
    }
}