use super::*;
use core::ops::*;

macro_rules! define_scalar_binary_operator_impl {
    ($t:ty, $op:tt, $trait_name:ident, $method_name:ident, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {

        // T op T
        impl $trait_name<$t> for $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs
                ] }
            }
        }

        impl $trait_name<$t> for $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs
                ] }
            }
        }

        impl $trait_name<$t> for $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs,
                    self.w() $op rhs
                ] }
            }
        }

        //T op &T
        impl<'a> $trait_name<&'a $t> for $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $t) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs
                ] }
            }
        }

        impl<'a> $trait_name<&'a $t> for $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $t) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs
                ] }
            }
        }

        impl<'a> $trait_name<&'a $t> for $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $t) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs,
                    self.w() $op rhs
                ] }
            }
        }

        //&T op T
        impl<'a> $trait_name<$t> for &'a $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs
                ] }
            }
        }

        impl<'a> $trait_name<$t> for &'a $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs
                ] }
            }
        }

        impl<'a> $trait_name<$t> for &'a $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: $t) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs,
                    self.w() $op rhs
                ] }
            }
        }

        //&T op &T
        impl<'a, 'b> $trait_name<&'b $t> for &'a $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $t) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs
                ] }
            }
        }

        impl<'a, 'b> $trait_name<&'b $t> for &'a $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $t) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs
                ] }
            }
        }

        impl<'a, 'b> $trait_name<&'b $t> for &'a $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $t) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs,
                    self.y() $op rhs,
                    self.z() $op rhs,
                    self.w() $op rhs
                ] }
            }
        }
    };
}

// Floating point types

define_scalar_binary_operator_impl!(f32, +, Add, add, Vector2f32, Vector3f32, Vector4f32);
define_scalar_binary_operator_impl!(f64, +, Add, add, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_scalar_binary_operator_impl!(u8, +, Add, add, Vector2u8, Vector3u8, Vector4u8);
define_scalar_binary_operator_impl!(u16, +, Add, add, Vector2u16, Vector3u16, Vector4u16);
define_scalar_binary_operator_impl!(u32, +, Add, add, Vector2u32, Vector3u32, Vector4u32);
define_scalar_binary_operator_impl!(u64, +, Add, add, Vector2u64, Vector3u64, Vector4u64);

define_scalar_binary_operator_impl!(u128, +, Add, add, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_scalar_binary_operator_impl!(i8, +, Add, add, Vector2i8, Vector3i8, Vector4i8);
define_scalar_binary_operator_impl!(i16, +, Add, add, Vector2i16, Vector3i16, Vector4i16);
define_scalar_binary_operator_impl!(i32, +, Add, add, Vector2i32, Vector3i32, Vector4i32);
define_scalar_binary_operator_impl!(i64, +, Add, add, Vector2i64, Vector3i64, Vector4i64);

define_scalar_binary_operator_impl!(i128, +, Add, add, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_scalar_binary_operator_impl!(usize, +, Add, add, Vector2usz, Vector3usz, Vector4usz);
define_scalar_binary_operator_impl!(isize, +, Add, add, Vector2isz, Vector3isz, Vector4isz);

// Floating point types

define_scalar_binary_operator_impl!(f32, -, Sub, sub, Vector2f32, Vector3f32, Vector4f32);
define_scalar_binary_operator_impl!(f64, -, Sub, sub, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_scalar_binary_operator_impl!(u8, -, Sub, sub, Vector2u8, Vector3u8, Vector4u8);
define_scalar_binary_operator_impl!(u16, -, Sub, sub, Vector2u16, Vector3u16, Vector4u16);
define_scalar_binary_operator_impl!(u32, -, Sub, sub, Vector2u32, Vector3u32, Vector4u32);
define_scalar_binary_operator_impl!(u64, -, Sub, sub, Vector2u64, Vector3u64, Vector4u64);

define_scalar_binary_operator_impl!(u128, -, Sub, sub, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_scalar_binary_operator_impl!(i8, -, Sub, sub, Vector2i8, Vector3i8, Vector4i8);
define_scalar_binary_operator_impl!(i16, -, Sub, sub, Vector2i16, Vector3i16, Vector4i16);
define_scalar_binary_operator_impl!(i32, -, Sub, sub, Vector2i32, Vector3i32, Vector4i32);
define_scalar_binary_operator_impl!(i64, -, Sub, sub, Vector2i64, Vector3i64, Vector4i64);

define_scalar_binary_operator_impl!(i128, -, Sub, sub, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_scalar_binary_operator_impl!(usize, -, Sub, sub, Vector2usz, Vector3usz, Vector4usz);
define_scalar_binary_operator_impl!(isize, -, Sub, sub, Vector2isz, Vector3isz, Vector4isz);

// Floating point types

define_scalar_binary_operator_impl!(f32, *, Mul, mul, Vector2f32, Vector3f32, Vector4f32);
define_scalar_binary_operator_impl!(f64, *, Mul, mul, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_scalar_binary_operator_impl!(u8, *, Mul, mul, Vector2u8, Vector3u8, Vector4u8);
define_scalar_binary_operator_impl!(u16, *, Mul, mul, Vector2u16, Vector3u16, Vector4u16);
define_scalar_binary_operator_impl!(u32, *, Mul, mul, Vector2u32, Vector3u32, Vector4u32);
define_scalar_binary_operator_impl!(u64, *, Mul, mul, Vector2u64, Vector3u64, Vector4u64);

define_scalar_binary_operator_impl!(u128, *, Mul, mul, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_scalar_binary_operator_impl!(i8, *, Mul, mul, Vector2i8, Vector3i8, Vector4i8);
define_scalar_binary_operator_impl!(i16, *, Mul, mul, Vector2i16, Vector3i16, Vector4i16);
define_scalar_binary_operator_impl!(i32, *, Mul, mul, Vector2i32, Vector3i32, Vector4i32);
define_scalar_binary_operator_impl!(i64, *, Mul, mul, Vector2i64, Vector3i64, Vector4i64);

define_scalar_binary_operator_impl!(i128, *, Mul, mul, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_scalar_binary_operator_impl!(usize, *, Mul, mul, Vector2usz, Vector3usz, Vector4usz);
define_scalar_binary_operator_impl!(isize, *, Mul, mul, Vector2isz, Vector3isz, Vector4isz);
