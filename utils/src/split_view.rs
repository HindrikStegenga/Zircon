pub struct SplitView<'a, T: 'a> {
    item: &'a T,
    before: &'a [T],
    after: &'a [T],
}

impl<'a, T: 'a> SplitView<'a, T> {
    pub fn new(slice: &'a mut [T], index: usize) -> Option<SplitView<'a, T>> {
        if index > slice.len() {
            return None;
        }

        let (before, middle) = slice.split_at(index);
        let (middle, after) = middle.split_at(1);
        Some(SplitView {
            before,
            item: &middle[0],
            after,
        })
    }

    /// Continues iterating over the slice until the slice is empty.
    pub fn for_each(slice: &'a [T], mut f: impl FnMut(SplitView<'a, T>)) {
        for i in 0..slice.len() {
            let (before, middle) = slice.split_at(i);
            let (middle, after) = middle.split_at(1);
            (f)(SplitView {
                item: &middle[0],
                before,
                after,
            })
        }
    }

    /// Continues iterating over the slice until the slice is empty or until the closure returns `false`.
    /// The function returns `true` if the slice was iterated to the end.
    /// The function returns `false` if the closure returned false;
    pub fn for_each_until(slice: &[T], mut f: impl FnMut(SplitView<'_, T>) -> bool) -> bool {
        for i in 0usize..slice.len() {
            let (before, middle) = slice.split_at(i);
            let (middle, after) = middle.split_at(1);
            if !(f)(SplitView {
                item: &middle[0],
                before,
                after,
            }) {
                return false;
            }
        }
        true
    }

    /// Continues iterating over the slice until the slice is empty or until the closure returns `Err(E)`.
    /// The function returns `Ok(())` if the slice was iterated to the end.
    /// The function returns `Err(E)` if the closure returned false;
    pub fn for_each_until_error<E>(
        slice: &[T],
        mut f: impl FnMut(SplitView<'_, T>) -> Result<(), E>,
    ) -> Result<(), E> {
        for i in 0usize..slice.len() {
            let (before, middle) = slice.split_at(i);
            let (middle, after) = middle.split_at(1);
            match (f)(SplitView {
                item: &middle[0],
                before,
                after,
            }) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Gets a simultaneous reference to all the components being splitted over.
    pub fn components(&self) -> (&[T], &T, &[T]) {
        (self.before, self.item, self.after)
    }

    /// Get a reference to the current item's before.
    pub fn before(&self) -> &[T] {
        self.before
    }

    /// Get a reference to the current item's after.
    pub fn after(&self) -> &[T] {
        self.after
    }

    /// Get a reference to the current item's item.
    pub fn item(&self) -> &T {
        self.item
    }
}

impl<'a, T> AsRef<T> for SplitView<'a, T> {
    fn as_ref(&self) -> &T {
        self.item
    }
}

pub struct SplitViewMut<'a, T: 'a> {
    before: &'a mut [T],
    item: &'a mut T,
    after: &'a mut [T],
}

impl<'a, T: 'a> SplitViewMut<'a, T> {
    pub fn new(slice: &mut [T], index: usize) -> Option<SplitViewMut<'_, T>> {
        if index > slice.len() {
            return None;
        }

        let (before, middle) = slice.split_at_mut(index);
        let (middle, after) = middle.split_at_mut(1);
        Some(SplitViewMut {
            before,
            item: &mut middle[0],
            after,
        })
    }

    /// Continues iterating over the slice until the slice is empty.
    pub fn for_each(slice: &mut [T], mut f: impl FnMut(SplitViewMut<'_, T>)) {
        for i in 0..slice.len() {
            let (before, middle) = slice.split_at_mut(i);
            let (middle, after) = middle.split_at_mut(1);
            (f)(SplitViewMut {
                item: &mut middle[0],
                before,
                after,
            });
        }
    }

    /// Continues iterating over the slice until the slice is empty or until the closure returns `false`.
    /// The function returns `true` if the slice was iterated to the end.
    /// The function returns `false` if the closure returned false;
    pub fn for_each_until(slice: &mut [T], mut f: impl FnMut(SplitViewMut<'_, T>) -> bool) -> bool {
        for i in 0usize..slice.len() {
            let (before, middle) = slice.split_at_mut(i);
            let (middle, after) = middle.split_at_mut(1);
            if !(f)(SplitViewMut {
                item: &mut middle[0],
                before,
                after,
            }) {
                return false;
            }
        }
        true
    }

    /// Continues iterating over the slice until the slice is empty or until the closure returns `Err(E)`.
    /// The function returns `Ok(())` if the slice was iterated to the end.
    /// The function returns `Err(E)` if the closure returned false;
    pub fn for_each_until_error<E>(
        slice: &mut [T],
        mut f: impl FnMut(SplitViewMut<'_, T>) -> Result<(), E>,
    ) -> Result<(), E> {
        for i in 0usize..slice.len() {
            let (before, middle) = slice.split_at_mut(i);
            let (middle, after) = middle.split_at_mut(1);
            match (f)(SplitViewMut {
                item: &mut middle[0],
                before,
                after,
            }) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Gets a simultaneous reference to all the components being splitted over.
    pub fn components(&self) -> (&[T], &T, &[T]) {
        (self.before, self.item, self.after)
    }

    /// Gets a simultaneous mutable reference to all the components being splitted over.
    pub fn components_mut(&mut self) -> (&mut [T], &mut T, &mut [T]) {
        (self.before, self.item, self.after)
    }

    /// Get a mutable reference to the current item's before.
    pub fn before_mut(&mut self) -> &mut [T] {
        self.before
    }

    /// Get a mutable reference to the current item's after.
    pub fn after_mut(&mut self) -> &mut [T] {
        self.after
    }

    /// Get a mutable reference to the current item's item.
    pub fn item_mut(&mut self) -> &mut T {
        self.item
    }

    /// Get a reference to the current item's before.
    pub fn before(&self) -> &[T] {
        self.before
    }

    /// Get a reference to the current item's after.
    pub fn after(&self) -> &[T] {
        self.after
    }

    /// Get a reference to the current item's item.
    pub fn item(&self) -> &T {
        self.item
    }
}

impl<'a, T> AsRef<T> for SplitViewMut<'a, T> {
    fn as_ref(&self) -> &T {
        self.item
    }
}
impl<'a, T> AsMut<T> for SplitViewMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.item
    }
}
