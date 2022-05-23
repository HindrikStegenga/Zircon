pub(crate) fn build_array<T, const N: usize>(f: impl Fn(usize) -> T) -> [T; N] {
    let mut counter = 0;
    [(); N].map(|_| {
        let t = (f)(counter);
        counter += 1;
        t
    })
}
