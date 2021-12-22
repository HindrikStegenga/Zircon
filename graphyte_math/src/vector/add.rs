use core::ops::{Add, AddAssign};
use crate::Vector;

// Following impls are for Vector<T, N> + Vector<T, N>

// &T op &U
impl<'a, 'b, T, const N: usize> Add<&'a Vector<T, N>> for &'b Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs[i]);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Add<Vector<T, N>> for &'a Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs[i]);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Add<&'a Vector<T, N>> for Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs[i]);
        }
        return_value
    }
}

//T op U
impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        let mut return_value =  Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs[i]);
        }
        return_value
    }
}

// Following impls are for AddAssign

impl<'a, T, const N: usize> AddAssign<&'a Vector<T, N>> for Vector<T, N>
    where T: AddAssign<&'a T>
{
    fn add_assign(&mut self, rhs: &'a Vector<T, N>) {
        for i in 0..N {
            self.values[i].add_assign(&rhs[i]);
        }
    }
}

impl<T, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N>
    where T: AddAssign<T> + Copy
{
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        for i in 0..N {
            self.values[i].add_assign(rhs[i]);
        }
    }
}

// Following impls are for Vector<T, N> + T

//T op U
impl<T, const N: usize> Add<T> for Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: T) -> Self::Output {
        let mut return_value = Self::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs);
        }
        return_value
    }
}

//&T op U
impl<'a, T, const N: usize> Add<T> for &'a Vector<T, N>
    where T: Add<T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs);
        }
        return_value
    }
}

//T op &U
impl<'a, T, const N: usize> Add<&'a T> for Vector<T, N>
    where T: Add<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs);
        }
        return_value
    }
}

//&T op &U
impl<'a, 'b, T, const N: usize> Add<&'a T> for &'b Vector<T, N>
    where T: Add<&'a T, Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn add(self, rhs: &'a T) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self[i].add(rhs);
        }
        return_value
    }
}

// Following impls are for AddAssign

impl<'a, T, const N: usize> AddAssign<&'a T> for Vector<T, N>
    where T: AddAssign<&'a T>
{
    fn add_assign(&mut self, rhs: &'a T) {
        for i in 0..N {
            self.values[i].add_assign(&rhs);
        }
    }
}

impl<T, const N: usize> AddAssign<T> for Vector<T, N>
    where T: AddAssign<T> + Copy
{
    fn add_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.values[i].add_assign(rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3u;

    #[test]
    fn test_addition() {
        assert_eq!(Vec3u::from([0,1,2]), Vec3u::from([0, 0, 0]) + Vec3u::from([0, 1, 2]));
    }
}