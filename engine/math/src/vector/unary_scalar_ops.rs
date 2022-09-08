use super::*;
use core::ops::*;

impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        Vector::build(|i| -self[i])
    }
}

impl<T, const N: usize> Neg for &Vector<T, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        Vector::build(|i| -self[i])
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_negate() {
        let a = Vector::<i64, 3>::from_array([1, 2, 3]);
        assert_eq!(-a, Vector::<i64, 3>::from_array([-1, -2, -3]));
        assert_eq!(-(&a), Vector::<i64, 3>::from_array([-1, -2, -3]));
    }
}
