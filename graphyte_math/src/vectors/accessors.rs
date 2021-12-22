use super::*;

macro_rules! define_accessors_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl $v2_name {
            pub const fn x(&self) -> $t {
                self.values[0]
            }

            pub const fn y(&self) -> $t {
                self.values[1]
            }

            pub const fn xy(&self) -> $v2_name {
                *self
            }

            pub const fn yx(&self) -> $v2_name {
                $v2_name { values: [self.y(), self.x()] }
            }
        }
        impl $v3_name {
            pub const fn x(&self) -> $t {
                self.values[0]
            }

            pub const fn y(&self) -> $t {
                self.values[1]
            }

            pub const fn z(&self) -> $t {
                self.values[2]
            }

            pub const fn xy(&self) -> $v2_name {
                $v2_name { values: [self.x(), self.y()] }
            }

            pub const fn yx(&self) -> $v2_name {
                $v2_name { values: [self.y(), self.x()] }
            }

            pub const fn xyz(&self) -> $v3_name {
                *self
            }

            pub const fn zyx(&self) -> $v3_name {
                $v3_name { values: [self.z(), self.y(), self.x()] }
            }
        }
        impl $v4_name {
            pub const fn x(&self) -> $t {
                self.values[0]
            }

            pub const fn y(&self) -> $t {
                self.values[1]
            }

            pub const fn z(&self) -> $t {
                self.values[2]
            }

            pub const fn w(&self) -> $t {
                self.values[3]
            }

            pub const fn xy(&self) -> $v2_name {
                $v2_name { values: [self.x(), self.y()] }
            }

            pub const fn yx(&self) -> $v2_name {
                $v2_name { values: [self.y(), self.x()] }
            }

            pub const fn xyz(&self) -> $v3_name {
                $v3_name { values: [self.x(), self.y(), self.z()] }
            }

            pub const fn zyx(&self) -> $v3_name {
                $v3_name { values: [self.z(), self.y(), self.x()] }
            }

            pub const fn xyzw(&self) -> $v4_name {
                *self
            }

            pub const fn wzyx(&self) -> $v4_name {
                $v4_name { values: [self.w(), self.z(), self.y(), self.x()] }
            }
        }
    };
}


// Floating point types

define_accessors_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_accessors_impl!(f64, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_accessors_impl!(u8, Vector2u8, Vector3u8, Vector4u8);
define_accessors_impl!(u16, Vector2u16, Vector3u16, Vector4u16);
define_accessors_impl!(u32, Vector2u32, Vector3u32, Vector4u32);
define_accessors_impl!(u64, Vector2u64, Vector3u64, Vector4u64);

define_accessors_impl!(u128, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_accessors_impl!(i8, Vector2i8, Vector3i8, Vector4i8);
define_accessors_impl!(i16, Vector2i16, Vector3i16, Vector4i16);
define_accessors_impl!(i32, Vector2i32, Vector3i32, Vector4i32);
define_accessors_impl!(i64, Vector2i64, Vector3i64, Vector4i64);

define_accessors_impl!(i128, Vector2i128, Vector3i128, Vector4i128);

// sized types
define_accessors_impl!(usize, Vector2usz, Vector3usz, Vector4usz);
define_accessors_impl!(isize, Vector2isz, Vector3isz, Vector4isz);