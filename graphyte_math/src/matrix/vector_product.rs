use super::*;
use crate::vector::*;
use core::ops::*;

// 1 x N dot N x P => 1 x P
// T op T
impl<T, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Vector<T, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Vector<T, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|column| (0..N).map(|i| self[i] * rhs[[i, column]]).sum())
    }
}

// T op &T
impl<T, const N: usize, const P: usize> Mul<&Matrix<T, N, P>> for Vector<T, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Vector<T, P>;

    fn mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|column| (0..N).map(|i| self[i] * rhs[[i, column]]).sum())
    }
}

// &T op T
impl<T, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for &Vector<T, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Vector<T, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|column| (0..N).map(|i| self[i] * rhs[[i, column]]).sum())
    }
}

// &T op &T
impl<T, const N: usize, const P: usize> Mul<&Matrix<T, N, P>> for &Vector<T, N>
where
    T: Copy,
    T: Add<T>,
    T: Mul<T>,
    T: core::iter::Sum<<T as core::ops::Mul>::Output>,
{
    type Output = Vector<T, P>;

    fn mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        Self::Output::build(|column| (0..N).map(|i| self[i] * rhs[[i, column]]).sum())
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_vector_multiply() {
        // Test multiplying with the identity matrix.
        let a = Vec4u::one();
        let b = Mat4u::identity();
        assert_eq!(a * b, Vec4u::one());

        // Test multiplying with non identity matrix.
        let a = Vec4u::from_array([1, 2, 3, 4]);
        let b = Mat4u::from_arrays([[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]);

        assert_eq!(a * b, Vec4u::from_array([10, 20, 30, 40]));
    }
}
