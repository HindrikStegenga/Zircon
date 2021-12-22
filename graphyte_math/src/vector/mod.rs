use core::ops::{Index, IndexMut};
mod add;
mod sub;
mod mul;
mod neg;
mod accessors;
mod dot;
mod cross;

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct Vector<T, const N: usize> {
    values: [T; N]
}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {}

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone()
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self { values: [T::default(); N] }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self { values: value }
    }
}

impl<T, const N: usize> From<T> for Vector<T, N>
    where T: Copy
{
    fn from(value: T) -> Self {
        Self {
            values: [value; N]
        }
    }
}