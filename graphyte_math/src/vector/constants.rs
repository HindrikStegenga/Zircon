use super::*;

macro_rules! define_constants_impl {
    ($zero_value:expr, $one_value:expr, $t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl $v2_name {
            pub const fn zero() -> $v2_name {
                $v2_name {
                    values: [$zero_value, $zero_value],
                }
            }

            pub const fn one() -> $v2_name {
                $v2_name {
                    values: [$one_value, $one_value],
                }
            }

            pub const fn min() -> $v2_name {
                $v2_name {
                    values: [<$t>::MIN, <$t>::MIN],
                }
            }

            pub const fn max() -> $v2_name {
                $v2_name {
                    values: [<$t>::MAX, <$t>::MAX],
                }
            }
        }

        impl $v3_name {
            pub const fn zero() -> $v3_name {
                $v3_name {
                    values: [$zero_value, $zero_value, $zero_value],
                }
            }

            pub const fn one() -> $v3_name {
                $v3_name {
                    values: [$one_value, $one_value, $one_value],
                }
            }

            pub const fn min() -> $v3_name {
                $v3_name {
                    values: [<$t>::MIN, <$t>::MIN, <$t>::MIN],
                }
            }

            pub const fn max() -> $v3_name {
                $v3_name {
                    values: [<$t>::MAX, <$t>::MAX, <$t>::MAX],
                }
            }
        }

        impl $v4_name {
            pub const fn zero() -> $v4_name {
                $v4_name {
                    values: [$zero_value, $zero_value, $zero_value, $zero_value],
                }
            }

            pub const fn one() -> $v4_name {
                $v4_name {
                    values: [$one_value, $one_value, $one_value, $one_value],
                }
            }

            pub const fn min() -> $v4_name {
                $v4_name {
                    values: [<$t>::MIN, <$t>::MIN, <$t>::MIN, <$t>::MIN],
                }
            }

            pub const fn max() -> $v4_name {
                $v4_name {
                    values: [<$t>::MAX, <$t>::MAX, <$t>::MAX, <$t>::MAX],
                }
            }
        }
    };
}

// Floating point types

define_constants_impl!(0.0, 1.0, f32, Vector2f32, Vector3f32, Vector4f32);
define_constants_impl!(0.0, 1.0, f64, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_constants_impl!(0, 1, u8, Vector2u8, Vector3u8, Vector4u8);
define_constants_impl!(0, 1, u16, Vector2u16, Vector3u16, Vector4u16);
define_constants_impl!(0, 1, u32, Vector2u32, Vector3u32, Vector4u32);
define_constants_impl!(0, 1, u64, Vector2u64, Vector3u64, Vector4u64);

define_constants_impl!(0, 1, u128, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_constants_impl!(0, 1, i8, Vector2i8, Vector3i8, Vector4i8);
define_constants_impl!(0, 1, i16, Vector2i16, Vector3i16, Vector4i16);
define_constants_impl!(0, 1, i32, Vector2i32, Vector3i32, Vector4i32);
define_constants_impl!(0, 1, i64, Vector2i64, Vector3i64, Vector4i64);

define_constants_impl!(0, 1, i128, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_constants_impl!(0, 1, usize, Vector2usz, Vector3usz, Vector4usz);
define_constants_impl!(0, 1, isize, Vector2isz, Vector3isz, Vector4isz);
