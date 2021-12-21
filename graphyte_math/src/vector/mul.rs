use crate::*;
use core::ops::{Mul, MulAssign};

// Following impls are for Vector<T, N> + Vector<T, N>

impl<T, const N: usize> Mul for Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

impl<'a, T, const N: usize> Mul for &'a Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

impl<'a, T, const N: usize> MulAssign<&'a Vector<T, N>> for Vector<T, N>
    where T: MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: &'a Vector<T, N>) {
        for i in 0..N {
            self.values[i].mul_assign(&rhs[i]);
        }
    }
}

// Following impls are for Vector<T, N> + T

impl<T, const N: usize> Mul<T> for Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

impl<'a, T, const N: usize> Mul<T> for &'a Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

impl<'a, T, const N: usize> MulAssign<&'a T> for Vector<T, N>
    where T: MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: &'a T) {
        for i in 0..N {
            self.values[i].mul_assign(&rhs);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3u;

    #[test]
    fn test_multiplication() {
        assert_eq!(Vec3u::from([16, 64,144]), Vec3u::from([4, 8, 12]) * Vec3u::from([4, 8, 12]));
    }
}