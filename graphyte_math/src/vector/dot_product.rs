use super::*;

pub trait DotProduct<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

macro_rules! define_dot_product_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        // T op T
        impl DotProduct<$v2_name> for $v2_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v2_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y()
            }
        }

        impl DotProduct<$v3_name> for $v3_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v3_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z()
            }
        }

        impl DotProduct<$v4_name> for $v4_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v4_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z() +
                self.w() * rhs.w()
            }
        }

        //T op &T
        impl<'a> DotProduct<&'a $v2_name> for $v2_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'a $v2_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y()
            }
        }

        impl<'a> DotProduct<&'a $v3_name> for $v3_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'a $v3_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z()
            }
        }

        impl<'a> DotProduct<&'a $v4_name> for $v4_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'a $v4_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z() +
                self.w() * rhs.w()
            }
        }

        //&T op T
        impl<'a> DotProduct<$v2_name> for &'a $v2_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v2_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y()
            }
        }

        impl<'a> DotProduct<$v3_name> for &'a $v3_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v3_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z()
            }
        }

        impl<'a> DotProduct<$v4_name> for &'a $v4_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: $v4_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z() +
                self.w() * rhs.w()
            }
        }

        //&T op &T
        impl<'a, 'b> DotProduct<&'b $v2_name> for &'a $v2_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'b $v2_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y()
            }
        }

        impl<'a, 'b> DotProduct<&'b $v3_name> for &'a $v3_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'b $v3_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z()
            }
        }

        impl<'a, 'b> DotProduct<&'b $v4_name> for &'a $v4_name {
            type Output = $t;
            //noinspection ALL
            fn dot(self, rhs: &'b $v4_name) -> Self::Output {
                self.x() * rhs.x() +
                self.y() * rhs.y() +
                self.z() * rhs.z() +
                self.w() * rhs.w()
            }
        }
    };
}


// Floating point types

define_dot_product_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_dot_product_impl!(f64, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_dot_product_impl!(u8, Vector2u8, Vector3u8, Vector4u8);
define_dot_product_impl!(u16, Vector2u16, Vector3u16, Vector4u16);
define_dot_product_impl!(u32, Vector2u32, Vector3u32, Vector4u32);
define_dot_product_impl!(u64, Vector2u64, Vector3u64, Vector4u64);

define_dot_product_impl!(u128, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_dot_product_impl!(i8, Vector2i8, Vector3i8, Vector4i8);
define_dot_product_impl!(i16, Vector2i16, Vector3i16, Vector4i16);
define_dot_product_impl!(i32, Vector2i32, Vector3i32, Vector4i32);
define_dot_product_impl!(i64, Vector2i64, Vector3i64, Vector4i64);

define_dot_product_impl!(i128, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_dot_product_impl!(usize, Vector2usz, Vector3usz, Vector4usz);
define_dot_product_impl!(isize, Vector2isz, Vector3isz, Vector4isz);