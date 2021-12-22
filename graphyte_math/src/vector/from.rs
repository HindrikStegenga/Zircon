use super::*;

macro_rules! define_from_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl From<$t> for $v2_name {
            fn from(v: $t) -> Self {
                $v2_name {
                    values: [v, v]
                }
            }
        }

        impl From<$t> for $v3_name {
            fn from(v: $t) -> Self {
                $v3_name {
                    values: [v, v, v]
                }
            }
        }

        impl From<$t> for $v4_name {
            fn from(v: $t) -> Self {
                $v4_name {
                    values: [v, v, v, v]
                }
            }
        }

        impl<'a> From<&'a $t> for $v2_name {
            fn from(v: &'a $t) -> Self {
                $v2_name {
                    values: [*v, *v]
                }
            }
        }

        impl<'a> From<&'a $t> for $v3_name {
            fn from(v: &'a $t) -> Self {
                $v3_name {
                    values: [*v, *v, *v]
                }
            }
        }

        impl<'a> From<&'a $t> for $v4_name {
            fn from(v: &'a $t) -> Self {
                $v4_name {
                    values: [*v, *v, *v, *v]
                }
            }
        }
        
        impl From<[$t; 2]> for $v2_name {
            fn from(values: [$t; 2]) -> Self {
                $v2_name {
                    values
                }
            }
        }
        
        impl From<[$t; 3]> for $v3_name {
            fn from(values: [$t; 3]) -> Self {
                $v3_name {
                    values
                }
            }
        }
        
        impl From<[$t; 4]> for $v4_name {
            fn from(values: [$t; 4]) -> Self {
                $v4_name {
                    values
                }
            }
        }
        
        impl<'a> From<&'a [$t; 2]> for $v2_name {
            fn from(values: &'a [$t; 2]) -> Self {
                $v2_name {
                    values: *values
                }
            }
        }
        
        impl<'a> From<&'a [$t; 3]> for $v3_name {
            fn from(values: &'a [$t; 3]) -> Self {
                $v3_name {
                    values: *values
                }
            }
        }
        
        impl<'a> From<&'a [$t; 4]> for $v4_name {
            fn from(values: &'a [$t; 4]) -> Self {
                $v4_name {
                    values: *values
                }
            }
        }
    };
}

// Floating point types

define_from_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_from_impl!(f64, Vector2f64, Vector3f64, Vector4f64);

// Unsigned integer types

define_from_impl!(u8, Vector2u8, Vector3u8, Vector4u8);
define_from_impl!(u16, Vector2u16, Vector3u16, Vector4u16);
define_from_impl!(u32, Vector2u32, Vector3u32, Vector4u32);
define_from_impl!(u64, Vector2u64, Vector3u64, Vector4u64);

define_from_impl!(u128, Vector2u128, Vector3u128, Vector4u128);

// Signed integer types

define_from_impl!(i8, Vector2i8, Vector3i8, Vector4i8);
define_from_impl!(i16, Vector2i16, Vector3i16, Vector4i16);
define_from_impl!(i32, Vector2i32, Vector3i32, Vector4i32);
define_from_impl!(i64, Vector2i64, Vector3i64, Vector4i64);

define_from_impl!(i128, Vector2i128, Vector3i128, Vector4i128);

// Size types
define_from_impl!(usize, Vector2usz, Vector3usz, Vector4usz);
define_from_impl!(isize, Vector2isz, Vector3isz, Vector4isz);