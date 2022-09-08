use super::*;

macro_rules! impl_constants {
    ($t:ty, $zero:expr, $one:expr) => {
        impl<const N: usize> Vector<$t, N> {
            pub const fn zero() -> Self {
                Vector::from_scalar($zero)
            }

            pub const fn one() -> Self {
                Vector::from_scalar($one)
            }

            pub const fn min() -> Self {
                Vector::from_scalar(<$t>::MIN)
            }

            pub const fn max() -> Self {
                Vector::from_scalar(<$t>::MAX)
            }
        }
    };
}

impl_constants!(f32, 0.0, 1.0);
impl_constants!(f64, 0.0, 1.0);

impl_constants!(u8, 0, 1);
impl_constants!(u16, 0, 1);
impl_constants!(u32, 0, 1);
impl_constants!(u64, 0, 1);

impl_constants!(i8, 0, 1);
impl_constants!(i16, 0, 1);
impl_constants!(i32, 0, 1);
impl_constants!(i64, 0, 1);

impl_constants!(usize, 0, 1);
impl_constants!(isize, 0, 1);

//impl_constants!(u128, 0, 1);
//impl_constants!(i128, 0, 1);
