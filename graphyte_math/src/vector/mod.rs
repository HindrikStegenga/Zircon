use crate::functions::*;
use alloc::format;
use core::{
    any::type_name,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Index, IndexMut},
};

mod accessors;
mod binary_scalar_ops;
mod binary_vector_ops;
mod constants;
mod constructors;
mod dot_product;
mod from;
mod serialization;
mod unary_scalar_ops;

pub(crate) use serialization::FixedArrayVisitor;

/// Generic vector type.
/// This type is marked `repr(C)`.
/// It's stored in Row Major ordering.
/// Therefore it represents a 1 x N vector.
/// Where N is the amount of columns.
#[repr(C)]
pub struct Vector<T, const N: usize> {
    values: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn build(f: impl Fn(usize) -> T) -> Self {
        Vector {
            values: build_array(f),
        }
    }
}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl<T, const N: usize> Copy for Vector<T, N> where T: Clone + Copy {}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            values: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Hash for Vector<T, N>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.values.hash(state);
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        for i in 0..(N - 1) {
            write!(f, "{},", self[i])?;
        }
        write!(f, "{})", self[N - 1])
    }
}

impl<T, const N: usize> Debug for Vector<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Vector<{},{}>", type_name::<T>(), N))
            .field("values", &self.values)
            .finish()
    }
}

impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T, const N: usize> Eq for Vector<T, N> where T: PartialEq + Eq {}

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

pub type Vec2i = Vector<i32, 2>;
pub type Vec2u = Vector<u32, 2>;
pub type Vec2f = Vector<f32, 2>;

pub type Vec3i = Vector<i32, 3>;
pub type Vec3u = Vector<u32, 3>;
pub type Vec3f = Vector<f32, 3>;

pub type Vec4i = Vector<i32, 4>;
pub type Vec4u = Vector<u32, 4>;
pub type Vec4f = Vector<f32, 4>;
