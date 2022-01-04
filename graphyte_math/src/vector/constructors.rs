use super::*;

macro_rules! define_constructor_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl $v2_name {
            pub const fn from_scalar(v: $t) -> Self {
                $v2_name { values: [v, v] }
            }

            pub const fn new(x: $t, y: $t) -> Self {
                $v2_name { values: [x, y] }
            }
        }

        impl $v3_name {
            pub const fn from_scalar(v: $t) -> Self {
                $v3_name { values: [v, v, v] }
            }

            pub const fn new(x: $t, y: $t, z: $t) -> Self {
                $v3_name { values: [x, y, z] }
            }
        }

        impl $v4_name {
            pub const fn from_scalar(v: $t) -> Self {
                $v4_name {
                    values: [v, v, v, v],
                }
            }

            pub const fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                $v4_name {
                    values: [x, y, z, w],
                }
            }
        }
    };
}

// Floating point types

define_constructor_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_constructor_impl!(f64, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_constructor_impl!(u8, Vector2u8, Vector3u8, Vector4u8);
define_constructor_impl!(u16, Vector2u16, Vector3u16, Vector4u16);
define_constructor_impl!(u32, Vector2u32, Vector3u32, Vector4u32);
define_constructor_impl!(u64, Vector2u64, Vector3u64, Vector4u64);

define_constructor_impl!(u128, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_constructor_impl!(i8, Vector2i8, Vector3i8, Vector4i8);
define_constructor_impl!(i16, Vector2i16, Vector3i16, Vector4i16);
define_constructor_impl!(i32, Vector2i32, Vector3i32, Vector4i32);
define_constructor_impl!(i64, Vector2i64, Vector3i64, Vector4i64);

define_constructor_impl!(i128, Vector2i128, Vector3i128, Vector4i128);

// sized types
define_constructor_impl!(usize, Vector2usz, Vector3usz, Vector4usz);
define_constructor_impl!(isize, Vector2isz, Vector3isz, Vector4isz);
