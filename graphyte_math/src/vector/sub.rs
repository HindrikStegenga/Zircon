use crate::*;
use core::ops::{Sub, SubAssign};

// Following impls are for Vector<T, N> + Vector<T, N>

impl<T, const N: usize> Sub for Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

impl<'a, T, const N: usize> Sub for &'a Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

impl<'a, T, const N: usize> SubAssign<&'a Vector<T, N>> for Vector<T, N>
    where T: SubAssign<&'a T>
{
    fn sub_assign(&mut self, rhs: &'a Vector<T, N>) {
        for i in 0..N {
            self.values[i].sub_assign(&rhs[i]);
        }
    }
}

// Following impls are for Vector<T, N> + T

impl<T, const N: usize> Sub<T> for Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

impl<'a, T, const N: usize> Sub<T> for &'a Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

impl<'a, T, const N: usize> SubAssign<&'a T> for Vector<T, N>
    where T: SubAssign<&'a T>
{
    fn sub_assign(&mut self, rhs: &'a T) {
        for i in 0..N {
            self.values[i].sub_assign(&rhs);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3u;

    #[test]
    fn test_subtraction() {
        assert_eq!(Vec3u::from([3,5,1]), Vec3u::from([5, 8, 2]) - Vec3u::from([2, 3, 1]));
    }
}