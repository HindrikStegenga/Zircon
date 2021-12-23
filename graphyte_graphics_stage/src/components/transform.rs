use graphyte_math::*;
use graphyte_engine::ecs::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    position: Vec3f,
    rotation: Vec4f,
    scale: f32
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
        Transform { position, rotation, scale }
    }
}

impl Component for Transform {
    const NAME: &'static str = "Transform";
}