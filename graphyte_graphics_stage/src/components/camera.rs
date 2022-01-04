use graphyte_engine::ecs::*;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Deserialize, Serialize)]
pub enum CameraKind {
    Orthographic = 0,
    Perspective = 1,
}

pub struct Camera {
    kind: CameraKind,
}

impl Component for Camera {
    const NAME: &'static str = "Camera";
}
