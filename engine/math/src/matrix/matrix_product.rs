use super::*;
use core::ops::*;

// M x N dot N x P => M x P
// T op T
impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|row, column| (0..N).map(|i| self[[row, i]] * rhs[[i, column]]).sum())
    }
}

// M x N dot N x P => M x P
// T op &T
impl<'a, T, const M: usize, const N: usize, const P: usize> Mul<&'a Matrix<T, N, P>>
    for Matrix<T, M, N>
where
    T: Copy,
    T: Add<&'a T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|row, column| (0..N).map(|i| self[[row, i]] * rhs[[i, column]]).sum())
    }
}

// M x N dot N x P => M x P
// &T op T
impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for &Matrix<T, M, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|row, column| (0..N).map(|i| self[[row, i]] * rhs[[i, column]]).sum())
    }
}

// M x N dot N x P => M x P
// &T op &T
impl<'a, T, const M: usize, const N: usize, const P: usize> Mul<&'a Matrix<T, N, P>>
    for &Matrix<T, M, N>
where
    T: Copy,
    T: Add<&'a T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|row, column| (0..N).map(|i| self[[row, i]] * rhs[[i, column]]).sum())
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_matrix_multiply() {
        let a = Mat4u::identity();
        let b = Mat4u::identity();
        assert_eq!(a * b, Mat4u::identity());

        let a = Mat4u::from_arrays([[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]);
        let b = Mat4u::from_arrays([[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]);

        assert_eq!(
            a * b,
            Mat4u::from_arrays([
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40],
                [10, 20, 30, 40]
            ])
        );
    }
}
