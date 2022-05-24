use super::*;

macro_rules! impl_constants {
    ($t:ty, $zero:expr, $one:expr) => {
        impl<const R: usize, const C: usize> Matrix<$t, R, C> {
            pub const fn zero() -> Self {
                Matrix::from_scalar($zero)
            }

            pub const fn one() -> Self {
                Matrix::from_scalar($one)
            }

            pub const fn min() -> Self {
                Matrix::from_scalar(<$t>::MIN)
            }

            pub const fn max() -> Self {
                Matrix::from_scalar(<$t>::MAX)
            }
        }
        impl<const N: usize> Matrix<$t, N, N> {
            pub const fn identity() -> Self {
                let mut matrix = Self::zero();
                let mut c = 0;
                while c < N {
                    matrix.values[c][c] = $one;
                    c += 1;
                }
                matrix
            }
        }
    };
}

impl_constants!(f32, 0.0, 1.0);
impl_constants!(f64, 0.0, 1.0);

impl_constants!(u8, 0, 1);
impl_constants!(u16, 0, 1);
impl_constants!(u32, 0, 1);
impl_constants!(u64, 0, 1);

impl_constants!(i8, 0, 1);
impl_constants!(i16, 0, 1);
impl_constants!(i32, 0, 1);
impl_constants!(i64, 0, 1);

impl_constants!(usize, 0, 1);
impl_constants!(isize, 0, 1);

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Copy,
{
    pub fn transpose(&self) -> Matrix<T, C, R> {
        Matrix::<T, C, R>::build(|row, column| self[[column, row]])
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_transpose() {
        let a = Mat4u::from_arrays([
            [10, 20, 30, 40],
            [10, 20, 30, 40],
            [10, 20, 30, 40],
            [10, 20, 30, 40],
        ]);
        let b = a.transpose();
        assert_eq!(
            b,
            Mat4u::from_arrays([
                [10, 10, 10, 10],
                [20, 20, 20, 20],
                [30, 30, 30, 30],
                [40, 40, 40, 40],
            ])
        );
        let a = Matrix::from_arrays([[10, 20, 30, 40], [10, 20, 30, 40], [10, 20, 30, 40]]);
        let b = a.transpose();
        assert_eq!(
            b,
            Matrix::from_arrays([[10, 10, 10], [20, 20, 20], [30, 30, 30], [40, 40, 40],])
        );
    }
}
