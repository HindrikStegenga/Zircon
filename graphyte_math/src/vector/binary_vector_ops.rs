use super::*;
use core::ops::*;

macro_rules! define_vector_binary_operator_impl {
    ($op:tt, $trait_name:ident, $method_name:ident) => {

        // T op T
        impl<T, const N: usize> $trait_name<Vector<T, N>> for Vector<T, N>
        where T: $trait_name<Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: Vector<T, N>) -> Self::Output {
                Vector::build(|i| self[i] $op rhs[i])
            }
        }

        // T op &T
        impl<T, const N: usize> $trait_name<&Vector<T, N>> for Vector<T, N>
        where T: for<'c> $trait_name<T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: &Vector<T, N>) -> Self::Output {
                Vector::build(|i| self[i] $op rhs[i])
            }
        }

        // &T op T
        impl<T, const N: usize> $trait_name<Vector<T, N>> for &Vector<T, N>
        where T: $trait_name<T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: Vector<T, N>) -> Self::Output {
                Vector::build(|i| self[i] $op rhs[i])
            }
        }

        // &T op &T
        impl<T, const N: usize> $trait_name<&Vector<T,N>> for &Vector<T, N>
        where T: for<'c> $trait_name<&'c T, Output = T>, T: Copy
        {
            type Output = Vector<T, N>;
            fn $method_name(self, rhs: &Vector<T, N>) -> Self::Output {
                Vector::build(|i| self[i] $op &rhs[i])
            }
        }
    };
}

define_vector_binary_operator_impl!(+, Add, add);
define_vector_binary_operator_impl!(-, Sub, sub);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_binary_vector_ops() {
        let a = Vector::<i32, 4>::from_array([1, 2, 3, 4]);
        let b = Vector::<i32, 4>::from_array([5, 6, 7, 8]);
        assert_eq!(a + b, Vector::from_array([6, 8, 10, 12]));
        assert_eq!(a - b, Vector::from_array([-4, -4, -4, -4]));

        assert_eq!(&a + b, Vector::from_array([6, 8, 10, 12]));
        assert_eq!(&a - b, Vector::from_array([-4, -4, -4, -4]));

        assert_eq!(a + &b, Vector::from_array([6, 8, 10, 12]));
        assert_eq!(a - &b, Vector::from_array([-4, -4, -4, -4]));

        assert_eq!(&a + &b, Vector::from_array([6, 8, 10, 12]));
        assert_eq!(&a - &b, Vector::from_array([-4, -4, -4, -4]));
    }
}
