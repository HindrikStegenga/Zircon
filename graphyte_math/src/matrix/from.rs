use super::*;

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn from_arrays(values: [[T; C]; R]) -> Self {
        Self { values }
    }

    pub const fn from_scalar(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            values: [[value; C]; R],
        }
    }

    pub const fn to_arrays(&self) -> [[T; C]; R]
    where
        T: Copy,
    {
        self.values
    }

    pub const fn as_arrays(&self) -> &[[T; C]; R] {
        &self.values
    }

    pub fn row(&self, index: usize) -> [T; C]
    where
        T: Copy,
    {
        self.values[index]
    }

    pub fn column(&self, index: usize) -> [T; R]
    where
        T: Copy,
    {
        build_array(|counter| self.values[counter][index])
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(values: [[T; C]; R]) -> Self {
        Self::from_arrays(values)
    }
}

impl<T, const R: usize, const C: usize> From<&[[T; C]; R]> for Matrix<T, R, C>
where
    T: Copy,
{
    fn from(v: &[[T; C]; R]) -> Self {
        Self::from_arrays(*v)
    }
}

impl<T, const R: usize, const C: usize> From<T> for Matrix<T, R, C>
where
    T: Copy,
{
    fn from(v: T) -> Self {
        Self::from_scalar(v)
    }
}

impl<T, const R: usize, const C: usize> From<&T> for Matrix<T, R, C>
where
    T: Copy,
{
    fn from(v: &T) -> Self {
        Self::from_scalar(*v)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_from_conversions() {
        let m = Mat4u::from_arrays([
            [11, 12, 13, 14], // x1, y1, z1, w1,
            [21, 22, 23, 24], // x2, y2, z2, w2,
            [31, 32, 33, 34], // x3, y3, z3, w3,
            [41, 42, 43, 44], // x4, y4, z4, w4,
        ]);
        assert_eq!(m.row(0), [11, 12, 13, 14]);
        assert_eq!(m.row(2), [31, 32, 33, 34]);
        assert_eq!(m.column(3), [14, 24, 34, 44]);
        assert_eq!(m.column(2), [13, 23, 33, 43]);
    }
}
