use super::*;
use core::ops::*;

macro_rules! define_scalar_binary_operator_impl {
    ($op:tt, $trait_name:ident, $method_name:ident) => {
        impl<T, const N: usize> $trait_name<T> for Vector<T, N>
        where T: $trait_name<Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: T) -> Self::Output {
                Vector::build(|i| self[i] $op rhs)
            }
        }

        impl<T, const N: usize> $trait_name<&T> for Vector<T, N>
        where T: for<'c> $trait_name<&'c T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: &T) -> Self::Output {
                Vector::build(|i| self[i] $op rhs)
            }
        }

        impl<T, const N: usize> $trait_name<T> for &Vector<T, N>
        where T: $trait_name<T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: T) -> Self::Output {
                Vector::build(|i| self[i] $op rhs)
            }
        }

        impl<T, const N: usize> $trait_name<&T> for &Vector<T, N>
        where T: for<'c> $trait_name<&'c T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: &T) -> Self::Output {
                Vector::build(|i| self[i] $op rhs)
            }
        }
    };
}

define_scalar_binary_operator_impl!(+, Add, add);
define_scalar_binary_operator_impl!(-, Sub, sub);
define_scalar_binary_operator_impl!(*, Mul, mul);
define_scalar_binary_operator_impl!(/, Div, div);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_binary_scalar_ops() {
        let mut a = Vector::<i32, 4>::from_array([1, 2, 3, 4]);
        let b = 2;

        assert_eq!(a + b, Vector::<i32, 4>::from_array([3, 4, 5, 6]));
        assert_eq!(a + &b, Vector::<i32, 4>::from_array([3, 4, 5, 6]));
        assert_eq!(&a + b, Vector::<i32, 4>::from_array([3, 4, 5, 6]));
        assert_eq!(&a + &b, Vector::<i32, 4>::from_array([3, 4, 5, 6]));

        assert_eq!(a - b, Vector::<i32, 4>::from_array([-1, 0, 1, 2]));
        assert_eq!(a - &b, Vector::<i32, 4>::from_array([-1, 0, 1, 2]));
        assert_eq!(&a - b, Vector::<i32, 4>::from_array([-1, 0, 1, 2]));
        assert_eq!(&a - &b, Vector::<i32, 4>::from_array([-1, 0, 1, 2]));

        assert_eq!(a * b, Vector::<i32, 4>::from_array([2, 4, 6, 8]));
        assert_eq!(a * &b, Vector::<i32, 4>::from_array([2, 4, 6, 8]));
        assert_eq!(&a * b, Vector::<i32, 4>::from_array([2, 4, 6, 8]));
        assert_eq!(&a * &b, Vector::<i32, 4>::from_array([2, 4, 6, 8]));

        a = Vector::<i32, 4>::from_array([2, 4, 6, 8]);

        assert_eq!(a / b, Vector::<i32, 4>::from_array([1, 2, 3, 4]));
        assert_eq!(a / &b, Vector::<i32, 4>::from_array([1, 2, 3, 4]));
        assert_eq!(&a / b, Vector::<i32, 4>::from_array([1, 2, 3, 4]));
        assert_eq!(&a / &b, Vector::<i32, 4>::from_array([1, 2, 3, 4]));
    }
}
