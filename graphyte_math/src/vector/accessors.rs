use super::*;

macro_rules! impl_accessor {
    ($name:ident, $vec_len:expr, $idx:expr) => {
        impl<T> Vector<T, $vec_len>
        where
            T: Copy,
        {
            pub const fn $name(&self) -> T {
                self.values[$idx]
            }
        }
    };
    ($name:ident, $vec_len:expr, $vec_return_len:expr, [ $($index:expr),* ]) => {
        impl<T> Vector<T, $vec_len>
        where
            T: Copy,
        {
            pub const fn $name(&self) -> Vector<T, $vec_return_len> {
                Vector { values: [ $(self.values[$index]), *] }
            }
        }
    };
}

impl_accessor!(x, 1, 0);
impl_accessor!(x, 2, 0);
impl_accessor!(x, 3, 0);
impl_accessor!(x, 4, 0);

impl_accessor!(y, 2, 1);
impl_accessor!(y, 3, 1);
impl_accessor!(y, 4, 1);

impl_accessor!(z, 3, 2);
impl_accessor!(z, 4, 2);

impl_accessor!(w, 4, 3);

impl_accessor!(xy, 2, 2, [0, 1]);
impl_accessor!(xy, 3, 2, [0, 1]);
impl_accessor!(xy, 4, 2, [0, 1]);

impl_accessor!(yx, 2, 2, [1, 0]);
impl_accessor!(yx, 3, 2, [1, 0]);
impl_accessor!(yx, 4, 2, [1, 0]);

impl_accessor!(xyz, 3, 3, [0, 1, 2]);
impl_accessor!(xyz, 4, 3, [0, 1, 2]);

impl_accessor!(zyx, 3, 3, [2, 1, 0]);
impl_accessor!(zyx, 4, 3, [2, 1, 0]);

impl_accessor!(xyzw, 4, 4, [0, 1, 2, 3]);
impl_accessor!(wzyx, 4, 4, [3, 2, 1, 0]);
