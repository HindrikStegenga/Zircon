use super::*;
use core::ops::*;

macro_rules! impl_matrix_addition {
    ($lhs:ty, $rhs:ty) => {
        impl<T, const R: usize, const C: usize> Add<$rhs> for $lhs
        where
            T: Add<T, Output = T>,
            T: Copy,
        {
            type Output = Matrix<T, R, C>;

            fn add(self, rhs: $rhs) -> Self::Output {
                Matrix::build(|row, column| self[[row, column]] + rhs[[row, column]])
            }
        }
    };
}

impl_matrix_addition!(Matrix<T,R,C>, Matrix<T,R,C>);
impl_matrix_addition!(Matrix<T,R,C>, &Matrix<T,R,C>);
impl_matrix_addition!(&Matrix<T,R,C>, Matrix<T,R,C>);
impl_matrix_addition!(&Matrix<T,R,C>, &Matrix<T,R,C>);
