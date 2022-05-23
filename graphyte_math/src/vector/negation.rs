use super::*;
use core::ops::Neg;

macro_rules! define_vector_negation_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl Neg for $v2_name {
            type Output = $v2_name;
            fn neg(self) -> Self::Output {
                $v2_name {
                    values: [-self.x(), -self.y()],
                }
            }
        }
        impl<'a> Neg for &'a $v2_name {
            type Output = $v2_name;
            fn neg(self) -> Self::Output {
                $v2_name {
                    values: [-self.x(), -self.y()],
                }
            }
        }

        impl Neg for $v3_name {
            type Output = $v3_name;
            fn neg(self) -> Self::Output {
                $v3_name {
                    values: [-self.x(), -self.y(), -self.z()],
                }
            }
        }

        impl<'a> Neg for &'a $v3_name {
            type Output = $v3_name;
            fn neg(self) -> Self::Output {
                $v3_name {
                    values: [-self.x(), -self.y(), -self.z()],
                }
            }
        }

        impl Neg for $v4_name {
            type Output = $v4_name;
            fn neg(self) -> Self::Output {
                $v4_name {
                    values: [-self.x(), -self.y(), -self.z(), -self.w()],
                }
            }
        }

        impl<'a> Neg for &'a $v4_name {
            type Output = $v4_name;
            fn neg(self) -> Self::Output {
                $v4_name {
                    values: [-self.x(), -self.y(), -self.z(), -self.w()],
                }
            }
        }
    };
}

// Floating point types

define_vector_negation_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_vector_negation_impl!(f64, Vector2f64, Vector3f64, Vector4f64);

// Signed integer types

define_vector_negation_impl!(i8, Vector2i8, Vector3i8, Vector4i8);
define_vector_negation_impl!(i16, Vector2i16, Vector3i16, Vector4i16);
define_vector_negation_impl!(i32, Vector2i32, Vector3i32, Vector4i32);
define_vector_negation_impl!(i64, Vector2i64, Vector3i64, Vector4i64);

define_vector_negation_impl!(i128, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_vector_negation_impl!(isize, Vector2isz, Vector3isz, Vector4isz);
