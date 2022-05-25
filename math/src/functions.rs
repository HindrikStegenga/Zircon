pub(crate) fn build_array<T, const N: usize>(f: impl Fn(usize) -> T) -> [T; N] {
    let mut counter = 0;
    [(); N].map(|_| {
        let t = (f)(counter);
        counter += 1;
        t
    })
}

pub(crate) fn build_arrays<T, const R: usize, const C: usize>(
    f: impl Fn(usize, usize) -> T,
) -> [[T; R]; C] {
    let mut row_counter = 0;
    [(); C].map(|_| {
        let mut column_counter = 0;
        let t = [(); R].map(|_| {
            let t = (f)(row_counter, column_counter);
            column_counter += 1;
            t
        });
        row_counter += 1;
        t
    })
}
