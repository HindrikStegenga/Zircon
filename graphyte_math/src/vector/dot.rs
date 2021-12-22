use core::ops::{AddAssign, Mul};
use crate::Vector;

pub trait DotProduct<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}


impl<T, const N: usize> DotProduct<Vector<T, N>> for Vector<T, N>
    where T: AddAssign<T> + Mul<T, Output = T> + Default + Copy
{
    type Output = T;
    //noinspection ALL
    fn dot(self, rhs: Vector<T, N>) -> Self::Output {
        let mut product : T = Default::default();
        for i in 0..N {
            product.add_assign(self.values[i].mul(rhs[i]));
        }
        product
    }
}

impl<'a, T, const N: usize> DotProduct<&'a Vector<T, N>> for Vector<T, N>
    where T: AddAssign<T> + Mul<&'a T, Output = T> + Default + Copy
{
    type Output = T;
    //noinspection ALL
    fn dot(self, rhs: &'a Vector<T, N>) -> Self::Output {
        let mut product : T = Default::default();
        for i in 0..N {
            product.add_assign(self.values[i].mul(&rhs[i]));
        }
        product
    }
}

impl<'a, T, const N: usize> DotProduct<Vector<T, N>> for &'a Vector<T, N>
    where T: AddAssign<T> + Mul<T, Output = T> + Default + Copy
{
    type Output = T;
    //noinspection ALL
    fn dot(self, rhs: Vector<T, N>) -> Self::Output {
        let mut product : T = Default::default();
        for i in 0..N {
            product.add_assign(self.values[i].mul(rhs[i]));
        }
        product
    }
}

impl<'a, 'b, T, const N: usize> DotProduct<&'b Vector<T, N>> for &'a Vector<T, N>
    where T: AddAssign<T> + Mul<T, Output = T> + Default + Copy
{
    type Output = T;
    //noinspection ALL
    fn dot(self, rhs: &'b Vector<T, N>) -> Self::Output {
        let mut product : T = Default::default();
        for i in 0..N {
            product.add_assign(self.values[i].mul(rhs[i]));
        }
        product
    }
}