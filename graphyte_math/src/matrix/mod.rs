mod constants;

use serde::{Serialize, Deserialize};

macro_rules! define_matrix_defs {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident, { $($derive:ident),* }) => {
        /// 2 dimensional row major square matrix type.
        /// This type is marked `repr(C)`.
        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v2_name {
            values: [$t; 4]
        }
        /// 3 dimensional row major square matrix type.
        /// This type is marked `repr(C)`.
        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v3_name {
            values: [$t; 9]
        }
        /// 4 dimensional row major square matrix type.
        /// This type is marked `repr(C)`.
        #[repr(C)]
        #[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, $($derive), *)]
        pub struct $v4_name {
            values: [$t; 16]
        }
    };
}

// Floating point types

define_matrix_defs!(f32, Matrix2x2f32, Matrix3x3f32, Matrix4x4f32, {});
define_matrix_defs!(f64, Matrix2x2f64, Matrix3x3f64, Matrix4x4f64, {});

// Unsigned integer types

define_matrix_defs!(u8, Matrix2x2u8, Matrix3x3u8, Matrix4x4u8, { Eq });
define_matrix_defs!(u16, Matrix2x2u16, Matrix3x3u16, Matrix4x4u16, { Eq });
define_matrix_defs!(u32, Matrix2x2u32, Matrix3x3u32, Matrix4x4u32, { Eq });
define_matrix_defs!(u64, Matrix2x2u64, Matrix3x3u64, Matrix4x4u64, { Eq });

define_matrix_defs!(u128, Matrix2x2u128, Matrix3x3u128, Matrix4x4u128, { Eq });

// Signed integer types

define_matrix_defs!(i8, Matrix2x2i8, Matrix3x3i8, Matrix4x4i8, { Eq });
define_matrix_defs!(i16, Matrix2x2i16, Matrix3x3i16, Matrix4x4i16, { Eq });
define_matrix_defs!(i32, Matrix2x2i32, Matrix3x3i32, Matrix4x4i32, { Eq });
define_matrix_defs!(i64, Matrix2x2i64, Matrix3x3i64, Matrix4x4i64, { Eq });

define_matrix_defs!(i128, Matrix2x2i128, Matrix3x3i128, Matrix4x4i128, { Eq });

// Size types
define_matrix_defs!(usize, Matrix2x2usz, Matrix3x3usz, Matrix4x4usz, { Eq });
define_matrix_defs!(isize, Matrix2x2isz, Matrix3x3isz, Matrix4x4isz, { Eq });

// Type defs

pub type Mat2i = Matrix2x2i32;
pub type Mat2u = Matrix2x2u32;
pub type Mat2f = Matrix2x2f32;

pub type Mat3i = Matrix3x3i32;
pub type Mat3u = Matrix3x3u32;
pub type Mat3f = Matrix3x3f32;

pub type Mat4i = Matrix4x4i32;
pub type Mat4u = Matrix4x4u32;
pub type Mat4f = Matrix4x4f32;