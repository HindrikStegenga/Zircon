use crate::functions::*;
use alloc::format;
use core::{
    any::type_name,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Index, IndexMut},
};

mod binary_matrix_ops;
mod binary_scalar_ops;
mod common_constants;
mod common_constructors;
mod from;
mod matrix_product;
mod serialization;
mod vector_product;

/// A generic matrix type.
/// This type is marked `repr(C)`.
/// It's stored in Row Major ordering.
/// => `R` is the number of rows, or height.
/// => `C` is the number of columns, or width.
#[repr(C)]
pub struct Matrix<T, const R: usize, const C: usize = R> {
    values: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn build(f: impl Fn(usize, usize) -> T) -> Self {
        Matrix {
            values: build_arrays(f),
        }
    }
}

impl<T, const R: usize, const C: usize> Clone for Matrix<T, R, C>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl<T, const R: usize, const C: usize> Hash for Matrix<T, R, C>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.values.hash(state);
    }
}

impl<T, const R: usize, const C: usize> Copy for Matrix<T, R, C> where T: Clone + Copy {}

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            values: [[Default::default(); C]; R],
        }
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.0][index.1]
    }
}

impl<T, const R: usize, const C: usize> Index<[usize; 2]> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.values[index[0]][index[1]]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<[usize; 2]> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.values[index[0]][index[1]]
    }
}

impl<T, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index / C][index % R]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index / C][index % R]
    }
}

impl<T, const R: usize, const C: usize> PartialEq for Matrix<T, R, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T, const R: usize, const C: usize> Eq for Matrix<T, R, C> where T: PartialEq + Eq {}

impl<T, const R: usize, const C: usize> Debug for Matrix<T, R, C>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Matrix<{},{}, {}>", type_name::<T>(), R, C))
            .field("values", &self.values)
            .finish()
    }
}

impl<T, const R: usize, const C: usize> Display for Matrix<T, R, C>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        for r in 0..R {
            write!(f, "(")?;
            for c in 0..(C - 1) {
                write!(f, "{},", self[[r, c]])?;
            }
            write!(f, "{})", self[[r, C - 1]])?;
        }
        write!(f, "]")
    }
}

pub type Mat2i = Matrix<i32, 2, 2>;
pub type Mat2u = Matrix<u32, 2, 2>;
pub type Mat2f = Matrix<f32, 2, 2>;

pub type Mat3i = Matrix<i32, 3, 3>;
pub type Mat3u = Matrix<u32, 3, 3>;
pub type Mat3f = Matrix<f32, 3, 3>;

pub type Mat4i = Matrix<i32, 4, 4>;
pub type Mat4u = Matrix<u32, 4, 4>;
pub type Mat4f = Matrix<f32, 4, 4>;

mod tests {
    extern crate std;
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::println;

    #[test]
    fn test_indexing() {
        let m = Mat4u::from_arrays([
            [11, 12, 13, 14], // x1, y1, z1, w1,
            [21, 22, 23, 24], // x2, y2, z2, w2,
            [31, 32, 33, 34], // x3, y3, z3, w3,
            [41, 42, 43, 44], // x4, y4, z4, w4,
        ]);
        println!("{}", m);
        // Test array indexing [RowIdx, ColumnIdx]
        assert_eq!(m[[0, 0]], 11);
        assert_eq!(m[[1, 2]], 23);
        assert_eq!(m[[3, 3]], 44);
        // Test tuple indexing (RowIdx, ColumnIdx)
        assert_eq!(m[(0, 0)], 11);
        assert_eq!(m[(1, 2)], 23);
        assert_eq!(m[(3, 3)], 44);
        // Test flat indexing
        assert_eq!(m[15], 44);
        assert_eq!(m[5], 22);
        assert_eq!(m[8], 31);
    }
}
