use super::*;
use core::ops::*;

pub trait DotProduct<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

// Vec<T> op Vec<T> => T
impl<T, const N: usize> DotProduct<Vector<T, N>> for Vector<T, N>
where
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: core::iter::Sum<T>,
    T: Copy,
{
    type Output = T;

    fn dot(self, rhs: Vector<T, N>) -> Self::Output {
        (0..N).map(|i| self[i] * rhs[i]).sum()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = Vector::<i32, 4>::from_array([1, 2, 3, 4]);
        let b = Vector::<i32, 4>::from_array([5, 6, 7, 8]);
        assert_eq!(a.dot(b), 70);
    }
}
