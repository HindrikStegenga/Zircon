use serde::{Serialize, Deserialize};
mod from;
mod constructors;
mod accessors;
mod properties;
mod binary_vector_ops;
mod binary_scalar_ops;
mod negation;

macro_rules! define_vector_defs {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident, { $($derive:ident),* }) => {
        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v2_name {
            values: [$t; 2]
        }

        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v3_name {
            values: [$t; 3]
        }

        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v4_name {
            values: [$t; 4]
        }
    };
}

// Floating point types

define_vector_defs!(f32, Vector2f32, Vector3f32, Vector4f32, {});
define_vector_defs!(f64, Vector2f64, Vector3f64, Vector4f64, {});

// Unsigned integer types

define_vector_defs!(u8, Vector2u8, Vector3u8, Vector4u8, { Eq });
define_vector_defs!(u16, Vector2u16, Vector3u16, Vector4u16, { Eq });
define_vector_defs!(u32, Vector2u32, Vector3u32, Vector4u32, { Eq });
define_vector_defs!(u64, Vector2u64, Vector3u64, Vector4u64, { Eq });

define_vector_defs!(u128, Vector2u128, Vector3u128, Vector4u128, { Eq });

// Signed integer types

define_vector_defs!(i8, Vector2i8, Vector3i8, Vector4i8, { Eq });
define_vector_defs!(i16, Vector2i16, Vector3i16, Vector4i16, { Eq });
define_vector_defs!(i32, Vector2i32, Vector3i32, Vector4i32, { Eq });
define_vector_defs!(i64, Vector2i64, Vector3i64, Vector4i64, { Eq });

define_vector_defs!(i128, Vector2i128, Vector3i128, Vector4i128, { Eq });

// Size types
define_vector_defs!(usize, Vector2usz, Vector3usz, Vector4usz, { Eq });
define_vector_defs!(isize, Vector2isz, Vector3isz, Vector4isz, { Eq });

// Type defs

pub type Vec2i = Vector2i32;
pub type Vec2u = Vector2u32;
pub type Vec2f = Vector2f32;

pub type Vec3i = Vector3i32;
pub type Vec3u = Vector3u32;
pub type Vec3f = Vector3f32;

pub type Vec4i = Vector4i32;
pub type Vec4u = Vector4u32;
pub type Vec4f = Vector4f32;