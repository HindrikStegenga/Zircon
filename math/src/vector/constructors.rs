use super::*;

// Constructs from components

macro_rules! impl_from_components {
    ($len:expr, $($label:ident), *) => {
        impl <T> Vector<T, $len> {
            pub const fn from_components($($label : T), *) -> Self {
                Self::from_array([$($label), *])
            }
        }
    };
}

impl_from_components!(4, x, y, z, w);
impl_from_components!(3, x, y, z);
impl_from_components!(2, x, y);

// Manual tuple conversions for small vectors

impl<T: Copy> Vector<T, 2> {
    pub const fn from_tuple(tuple: (T, T)) -> Self {
        Self::from_array([tuple.0, tuple.1])
    }

    pub const fn to_tuple(&self) -> (T, T) {
        (self.values[0], self.values[1])
    }
}

impl<T: Copy> Vector<T, 3> {
    pub const fn from_tuple(tuple: (T, T, T)) -> Self {
        Self::from_array([tuple.0, tuple.1, tuple.2])
    }

    pub const fn to_tuple(&self) -> (T, T, T) {
        (self.values[0], self.values[1], self.values[2])
    }
}

impl<T: Copy> Vector<T, 4> {
    pub const fn from_tuple(tuple: (T, T, T, T)) -> Self {
        Self::from_array([tuple.0, tuple.1, tuple.2, tuple.3])
    }

    pub const fn to_tuple(&self) -> (T, T, T, T) {
        (
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
        )
    }
}

// From / Into implementations for small vectors.

impl<T> From<(T, T)> for Vector<T, 2>
where
    T: Copy,
{
    fn from(value: (T, T)) -> Self {
        Self::from_tuple(value)
    }
}

impl<T> From<(T, T, T)> for Vector<T, 3>
where
    T: Copy,
{
    fn from(value: (T, T, T)) -> Self {
        Self::from_tuple(value)
    }
}

impl<T> From<(T, T, T, T)> for Vector<T, 4>
where
    T: Copy,
{
    fn from(value: (T, T, T, T)) -> Self {
        Self::from_tuple(value)
    }
}
