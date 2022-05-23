use super::Vector;

macro_rules! generate_accessors {
    ($name:ident, $idx:expr, [ $n:expr ]) => {
        impl<T> Vector<T, $n>
        where
            T: Copy,
        {
            pub const fn $name(&self) -> T {
                self.values[$idx]
            }
        }
    };
    ($name:ident, $idx:expr, [ $c_n:expr, $($n:expr), *]) => {
        generate_accessors!($name, $idx, [$c_n]);
        generate_accessors!($name, $idx, [$($n), *]);
    };
}

generate_accessors!(
    x,
    0,
    [
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9, 8, 7, 6, 5, 4, 3, 2, 1
    ]
);

generate_accessors!(
    y,
    1,
    [
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9, 8, 7, 6, 5, 4, 3, 2
    ]
);

generate_accessors!(
    z,
    2,
    [
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9, 8, 7, 6, 5, 4, 3
    ]
);

generate_accessors!(
    w,
    3,
    [
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
        9, 8, 7, 6, 5, 4
    ]
);
