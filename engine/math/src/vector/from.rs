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

    pub const fn to_array(&self) -> [T; N]
    where
        T: Copy,
    {
        self.values
    }

    pub const fn as_array(&self) -> &[T; N] {
        &self.values
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(values: [T; N]) -> Self {
        Self::from_array(values)
    }
}

impl<T, const N: usize> From<&[T; N]> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: &[T; N]) -> Self {
        Self::from_array(*v)
    }
}

impl<T, const N: usize> From<T> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: T) -> Self {
        Self::from_scalar(v)
    }
}

impl<T, const N: usize> From<&T> for Vector<T, N>
where
    T: Copy,
{
    fn from(v: &T) -> Self {
        Self::from_scalar(*v)
    }
}
