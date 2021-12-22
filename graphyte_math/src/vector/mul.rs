use core::ops::{Mul, MulAssign};
use crate::Vector;

// Following impls are for Vector<T, N> + Vector<T, N>

// &T op &U
impl<'a, 'b, T, const N: usize> Mul<&'a Vector<T, N>> for &'b Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Mul<Vector<T, N>> for &'a Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Mul<&'a Vector<T, N>> for Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

//T op U
impl<T, const N: usize> Mul<Vector<T, N>> for Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value =  Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs[i]);
        }
        return_value
    }
}

// Following impls are for MulAssign

impl<'a, T, const N: usize> MulAssign<&'a Vector<T, N>> for Vector<T, N>
    where T: MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: &'a Vector<T, N>) {
        for i in 0..N {
            self.values[i].mul_assign(&rhs[i]);
        }
    }
}

impl<T, const N: usize> MulAssign<Vector<T, N>> for Vector<T, N>
    where T: MulAssign<T> + Copy
{
    fn mul_assign(&mut self, rhs: Vector<T, N>) {
        for i in 0..N {
            self.values[i].mul_assign(rhs[i]);
        }
    }
}

// Following impls are for Vector<T, N> + T

//T op U
impl<T, const N: usize> Mul<T> for Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: T) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Mul<T> for &'a Vector<T, N>
    where T: Mul<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Mul<&'a T> for Vector<T, N>
    where T: Mul<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

//&T op &U
impl<'a, 'b, T, const N: usize> Mul<&'a T> for &'b Vector<T, N>
    where T: Mul<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn mul(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].mul(rhs);
        }
        return_value
    }
}

// Following impls are for MulAssign

impl<'a, T, const N: usize> MulAssign<&'a T> for Vector<T, N>
    where T: MulAssign<&'a T>
{
    fn mul_assign(&mut self, rhs: &'a T) {
        for i in 0..N {
            self.values[i].mul_assign(&rhs);
        }
    }
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N>
    where T: MulAssign<T> + Copy
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.values[i].mul_assign(rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3u;

    #[test]
    fn test_multiplication() {
        assert_eq!(Vec3u::from([1,4,9]), Vec3u::from([1, 2, 3]) * Vec3u::from([1, 2, 3]));
    }
}