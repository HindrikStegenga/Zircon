use super::*;
use core::ops::*;

// T op T
impl<T, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C>
where
    T: Mul<T, Output = T>,
    T: Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix::build(|r, c| self[[r, c]] * rhs)
    }
}

// &T op T
impl<T, const R: usize, const C: usize> Mul<T> for &Matrix<T, R, C>
where
    T: Mul<T, Output = T>,
    T: Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix::build(|r, c| self[[r, c]] * rhs)
    }
}

// T op &T
impl<'a, T, const R: usize, const C: usize> Mul<&'a T> for Matrix<T, R, C>
where
    T: Mul<&'a T, Output = T>,
    T: Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        Matrix::build(|r, c| self[[r, c]] * rhs)
    }
}

// &T op &T
impl<'a, T, const R: usize, const C: usize> Mul<&'a T> for &Matrix<T, R, C>
where
    T: Mul<&'a T, Output = T>,
    T: Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        Matrix::build(|r, c| self[[r, c]] * rhs)
    }
}

macro_rules! impl_left_mul {
    ([$t:ty, $lhs:ty, $rhs:ty]) => {
        impl<const R: usize, const C: usize> Mul<$rhs> for $lhs {
            type Output = Matrix<$t, R, C>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                rhs * self
            }
        }
    };
    ($($t:ty),*) => {
        $(
            // T op T
            impl_left_mul!([$t, $t, Matrix<$t, R, C>]);
            // &T op T
            impl_left_mul!([$t, &$t, Matrix<$t, R, C>]);
            // T op &T
            impl_left_mul!([$t, $t, &Matrix<$t, R, C>]);
            // &T op &T
            impl_left_mul!([$t, &$t, &Matrix<$t, R, C>]);
        ) *
    };
}

impl_left_mul!(f32, f64, u8, u16, u32, u64, i8, i16, i32, i64);
