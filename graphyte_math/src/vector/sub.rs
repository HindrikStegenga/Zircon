use core::ops::{Sub, SubAssign};
use crate::Vector;

// Following impls are for Vector<T, N> + Vector<T, N>

// &T op &U
impl<'a, 'b, T, const N: usize> Sub<&'a Vector<T, N>> for &'b Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Sub<Vector<T, N>> for &'a Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Sub<&'a Vector<T, N>> for Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

//T op U
impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value =  Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs[i]);
        }
        return_value
    }
}

// Following impls are for SubAssign

impl<'a, T, const N: usize> SubAssign<&'a Vector<T, N>> for Vector<T, N>
    where T: SubAssign<&'a T>
{
    fn sub_assign(&mut self, rhs: &'a Vector<T, N>) {
        for i in 0..N {
            self.values[i].sub_assign(&rhs[i]);
        }
    }
}

impl<T, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N>
    where T: SubAssign<T> + Copy
{
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        for i in 0..N {
            self.values[i].sub_assign(rhs[i]);
        }
    }
}

// Following impls are for Vector<T, N> + T

//T op U
impl<T, const N: usize> Sub<T> for Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: T) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Sub<T> for &'a Vector<T, N>
    where T: Sub<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Sub<&'a T> for Vector<T, N>
    where T: Sub<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

//&T op &U
impl<'a, 'b, T, const N: usize> Sub<&'a T> for &'b Vector<T, N>
    where T: Sub<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn sub(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].sub(rhs);
        }
        return_value
    }
}

// Following impls are for SubAssign

impl<'a, T, const N: usize> SubAssign<&'a T> for Vector<T, N>
    where T: SubAssign<&'a T>
{
    fn sub_assign(&mut self, rhs: &'a T) {
        for i in 0..N {
            self.values[i].sub_assign(&rhs);
        }
    }
}

impl<T, const N: usize> SubAssign<T> for Vector<T, N>
    where T: SubAssign<T> + Copy
{
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.values[i].sub_assign(rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3u;

    #[test]
    fn test_subtraction() {
        assert_eq!(Vec3u::from([0,1,2]), Vec3u::from([0, 2, 4]) - Vec3u::from([0, 1, 2]));
    }
}