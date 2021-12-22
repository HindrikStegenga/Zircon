use core::ops::Neg;
use crate::Vector;

impl<T, const N: usize> Neg for Vector<T, N>
    where T: Neg<Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn neg(self) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self.values[i].neg();
        }
        return_value
    }
}

impl<'a, T, const N: usize> Neg for &'a Vector<T, N>
    where T: Neg<Output = T> + Default + Copy
{
    type Output = Vector<T, N>;
    //noinspection ALL
    fn neg(self) -> Self::Output {
        let mut return_value = Vector::<T, N>::default();
        for i in 0..N {
            return_value[i] = self.values[i].neg();
        }
        return_value
    }
}