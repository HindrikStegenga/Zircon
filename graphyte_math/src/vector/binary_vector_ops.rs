use super::*;
use std::ops::*;

macro_rules! define_vector_binary_operator_impl {
    ($op:tt, $trait_name:ident, $method_name:ident, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {

        // T op T
        impl $trait_name<$v2_name> for $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v2_name) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y()
                ] }
            }
        }

        impl $trait_name<$v3_name> for $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v3_name) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z()
                ] }
            }
        }

        impl $trait_name<$v4_name> for $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v4_name) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z(),
                    self.w() $op rhs.w()
                ] }
            }
        }

        //T op &T
        impl<'a> $trait_name<&'a $v2_name> for $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $v2_name) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y()
                ] }
            }
        }

        impl<'a> $trait_name<&'a $v3_name> for $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $v3_name) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z()
                ] }
            }
        }

        impl<'a> $trait_name<&'a $v4_name> for $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'a $v4_name) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z(),
                    self.w() $op rhs.w()
                ] }
            }
        }

        //&T op T
        impl<'a> $trait_name<$v2_name> for &'a $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v2_name) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y()
                ] }
            }
        }

        impl<'a> $trait_name<$v3_name> for &'a $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v3_name) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z()
                ] }
            }
        }

        impl<'a> $trait_name<$v4_name> for &'a $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: $v4_name) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z(),
                    self.w() $op rhs.w()
                ] }
            }
        }

        //&T op &T
        impl<'a, 'b> $trait_name<&'b $v2_name> for &'a $v2_name {
            type Output = $v2_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $v2_name) -> Self::Output {
                $v2_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y()
                ] }
            }
        }

        impl<'a, 'b> $trait_name<&'b $v3_name> for &'a $v3_name {
            type Output = $v3_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $v3_name) -> Self::Output {
                $v3_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z()
                ] }
            }
        }

        impl<'a, 'b> $trait_name<&'b $v4_name> for &'a $v4_name {
            type Output = $v4_name;
            //noinspection ALL
            fn $method_name(self, rhs: &'b $v4_name) -> Self::Output {
                $v4_name { values: [
                    self.x() $op rhs.x(),
                    self.y() $op rhs.y(),
                    self.z() $op rhs.z(),
                    self.w() $op rhs.w()
                ] }
            }
        }
    };
}

// Floating point types

define_vector_binary_operator_impl!(+, Add, add, Vector2f32, Vector3f32, Vector4f32);
define_vector_binary_operator_impl!(+, Add, add, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_vector_binary_operator_impl!(+, Add, add, Vector2u8, Vector3u8, Vector4u8);
define_vector_binary_operator_impl!(+, Add, add, Vector2u16, Vector3u16, Vector4u16);
define_vector_binary_operator_impl!(+, Add, add, Vector2u32, Vector3u32, Vector4u32);
define_vector_binary_operator_impl!(+, Add, add, Vector2u64, Vector3u64, Vector4u64);

define_vector_binary_operator_impl!(+, Add, add, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_vector_binary_operator_impl!(+, Add, add, Vector2i8, Vector3i8, Vector4i8);
define_vector_binary_operator_impl!(+, Add, add, Vector2i16, Vector3i16, Vector4i16);
define_vector_binary_operator_impl!(+, Add, add, Vector2i32, Vector3i32, Vector4i32);
define_vector_binary_operator_impl!(+, Add, add, Vector2i64, Vector3i64, Vector4i64);

define_vector_binary_operator_impl!(+, Add, add, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_vector_binary_operator_impl!(+, Add, add, Vector2usz, Vector3usz, Vector4usz);
define_vector_binary_operator_impl!(+, Add, add, Vector2isz, Vector3isz, Vector4isz);

// Floating point types

define_vector_binary_operator_impl!(-, Sub, sub, Vector2f32, Vector3f32, Vector4f32);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_vector_binary_operator_impl!(-, Sub, sub, Vector2u8, Vector3u8, Vector4u8);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2u16, Vector3u16, Vector4u16);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2u32, Vector3u32, Vector4u32);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2u64, Vector3u64, Vector4u64);

define_vector_binary_operator_impl!(-, Sub, sub, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_vector_binary_operator_impl!(-, Sub, sub, Vector2i8, Vector3i8, Vector4i8);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2i16, Vector3i16, Vector4i16);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2i32, Vector3i32, Vector4i32);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2i64, Vector3i64, Vector4i64);

define_vector_binary_operator_impl!(-, Sub, sub, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_vector_binary_operator_impl!(-, Sub, sub, Vector2usz, Vector3usz, Vector4usz);
define_vector_binary_operator_impl!(-, Sub, sub, Vector2isz, Vector3isz, Vector4isz);