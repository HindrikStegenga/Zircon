use super::*;

macro_rules! define_constants_impl {
    ($zero_value:expr, $one_value:expr, $t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl $v2_name {
            pub const fn identity() -> $v2_name {
                $v2_name { values: [$one_value, $zero_value,
                                    $zero_value, $one_value ] }
            }

            pub const fn zero() -> $v2_name {
                $v2_name { values: [$zero_value; 4] }
            }

            pub const fn one() -> $v2_name {
                $v2_name { values: [$one_value; 4] }
            }

            pub const fn min() -> $v2_name {
                $v2_name { values: [<$t>::MIN; 4] }
            }

            pub const fn max() -> $v2_name {
                $v2_name { values: [<$t>::MAX; 4] }
            }
        }
        
        impl $v3_name {
            pub const fn identity() -> $v3_name {
                $v3_name { values: [$one_value, $zero_value, $zero_value,
                                    $zero_value, $one_value, $zero_value,
                                    $zero_value, $zero_value, $one_value
                ] }
            }

            pub const fn zero() -> $v3_name {
                $v3_name { values: [$zero_value; 9] }
            }

            pub const fn one() -> $v3_name {
                $v3_name { values: [$one_value; 9] }
            }

            pub const fn min() ->$v3_name {
                $v3_name { values: [<$t>::MIN; 9] }
            }

            pub const fn max() ->$v3_name {
                $v3_name { values: [<$t>::MAX; 9] }
            }
        }
        
        impl $v4_name {
            pub const fn identity() -> $v4_name {
                $v4_name { values: [$one_value, $zero_value, $zero_value, $zero_value,
                                    $zero_value, $one_value, $zero_value, $zero_value,
                                    $zero_value, $zero_value, $one_value, $zero_value,
                                    $zero_value, $zero_value, $zero_value, $one_value,
                ] }
            }

            pub const fn zero() -> $v4_name {
                $v4_name { values: [$zero_value; 16] }
            }

            pub const fn one() -> $v4_name {
                $v4_name { values: [$one_value; 16] }
            }

            pub const fn min() ->$v4_name {
                $v4_name { values: [<$t>::MIN; 16] }
            }

            pub const fn max() ->$v4_name {
                $v4_name { values: [<$t>::MAX; 16] }
            }
        }
    };
}

// Floating point types

define_constants_impl!(0.0, 1.0, f32, Matrix2x2f32, Matrix3x3f32, Matrix4x4f32);
define_constants_impl!(0.0, 1.0, f64, Matrix2x2f64, Matrix3x3f64, Matrix4x4f64);

// Unsigned integer types

define_constants_impl!(0, 1, u8, Matrix2x2u8, Matrix3x3u8, Matrix4x4u8);
define_constants_impl!(0, 1, u16, Matrix2x2u16, Matrix3x3u16, Matrix4x4u16);
define_constants_impl!(0, 1, u32, Matrix2x2u32, Matrix3x3u32, Matrix4x4u32);
define_constants_impl!(0, 1, u64, Matrix2x2u64, Matrix3x3u64, Matrix4x4u64);

define_constants_impl!(0, 1, u128, Matrix2x2u128, Matrix3x3u128, Matrix4x4u128);

// Signed integer types

define_constants_impl!(0, 1, i8, Matrix2x2i8, Matrix3x3i8, Matrix4x4i8);
define_constants_impl!(0, 1, i16, Matrix2x2i16, Matrix3x3i16, Matrix4x4i16);
define_constants_impl!(0, 1, i32, Matrix2x2i32, Matrix3x3i32, Matrix4x4i32);
define_constants_impl!(0, 1, i64, Matrix2x2i64, Matrix3x3i64, Matrix4x4i64);

define_constants_impl!(0, 1, i128, Matrix2x2i128, Matrix3x3i128, Matrix4x4i128);

// Size types
define_constants_impl!(0, 1, usize, Matrix2x2usz, Matrix3x3usz, Matrix4x4usz);
define_constants_impl!(0, 1, isize, Matrix2x2isz, Matrix3x3isz, Matrix4x4isz);