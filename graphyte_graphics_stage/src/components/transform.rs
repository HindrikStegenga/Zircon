use graphyte_engine::ecs::*;
use graphyte_math::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct Transform {
    position: Vec3f,
    scale: f32,
    rotation: Vec4f,
}

impl Transform {
    pub fn position(&self) -> Vec3f {
        self.position
    }
    pub fn rotation(&self) -> Vec4f {
        self.rotation
    }
    pub fn scale(&self) -> f32 {
        self.scale
    }
}

impl Transform {
    pub fn new(position: Vec3f, rotation: Vec4f, scale: f32) -> Self {
        Transform {
            position,
            rotation,
            scale,
        }
    }
}
