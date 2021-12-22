use crate::Vector;

impl<T> Vector<T, 1> {
    pub const fn x(&self) -> &T { &self.values[0] }
}

impl<T> Vector<T, 2> {
    pub const fn x(&self) -> &T { &self.values[0] }
    pub const fn y(&self) -> &T { &self.values[1] }
}

impl<T> Vector<T, 3> {
    pub const fn x(&self) -> &T { &self.values[0] }
    pub const fn y(&self) -> &T { &self.values[1] }
    pub const fn z(&self) -> &T { &self.values[2] }
}

impl<T> Vector<T, 4> {
    pub const fn x(&self) -> &T { &self.values[0] }
    pub const fn y(&self) -> &T { &self.values[1] }
    pub const fn z(&self) -> &T { &self.values[2] }
    pub const fn w(&self) -> &T { &self.values[3] }
}

impl<T: Copy> Vector<T, 4> {
    //noinspection ALL
    pub fn xyzw(&self) -> Vector<T, 4> {
        *self
    }
    //noinspection ALL
    pub fn xyz(&self) -> Vector<T, 3> {
        Vector::<T, 3>::from([self.values[0], self.values[1], self.values[2]])
    }
    //noinspection ALL
    pub fn xy(&self) -> Vector<T, 2> {
        Vector::<T, 2>::from([self.values[0], self.values[1]])
    }
}

impl<T: Copy> Vector<T, 3> {
    //noinspection ALL
    pub fn xyz(&self) -> Vector<T, 3> { *self }
    //noinspection ALL
    pub fn xy(&self) -> Vector<T, 2> {
        Vector::<T, 2>::from([self.values[0], self.values[1]])
    }
}

impl<T: Copy> Vector<T, 2> {
    //noinspection ALL
    pub fn xy(&self) -> Vector<T, 2> { *self }
}