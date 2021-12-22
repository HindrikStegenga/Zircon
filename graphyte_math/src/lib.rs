//#![no_std]

pub(crate) mod vector;
pub(crate) use vector::*;

pub type Vec2f = Vector<f32, 2>;
pub type Vec3f = Vector<f32, 3>;
pub type Vec4f = Vector<f32, 4>;

pub type Vec2i = Vector<i32, 2>;
pub type Vec3i = Vector<i32, 3>;
pub type Vec4i = Vector<i32, 4>;

pub type Vec2u = Vector<u32, 2>;
pub type Vec3u = Vector<u32, 3>;
pub type Vec4u = Vector<u32, 4>;