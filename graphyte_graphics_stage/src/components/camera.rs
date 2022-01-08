use crate::RenderPathType;
use graphyte_engine::ecs::*;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum CameraKind {
    Orthographic = 0,
    Perspective = 1,
}

pub struct Camera {
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
    pub fn new(kind: CameraKind, path: RenderPathType) -> Self {
        Camera { kind, path }
    }
}

impl Component for Camera {
    const NAME: &'static str = "Camera";
}
