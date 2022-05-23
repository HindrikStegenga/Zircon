use super::*;

impl<T, const N: usize> Vector<T, N> {
    pub const fn from_array(values: [T; N]) -> Self {
        Self { values }
    }

    pub const fn from_scalar(value: T) -> Self
    where
        T: Copy,
    {
        Self { values: [value; N] }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<T, const N: usize> From<&[T; N]> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: &[T; N]) -> Self {
        Self {
            values: [0; N].map(|i| v[i]),
        }
    }
}

impl<T, const N: usize> From<T> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: T) -> Self {
        Self { values: [v; N] }
    }
}

impl<T, const N: usize> From<&T> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: &T) -> Self {
        Self { values: [*v; N] }
    }
}
