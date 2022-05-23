use alloc::format;
use core::{
    any::type_name,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

mod accessors;
mod binary_scalar_ops;
mod binary_vector_ops;
mod dot_product;
mod from;
mod functions;
mod serialization;

#[repr(C)]
pub struct Vector<T, const N: usize> {
    values: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn build(f: impl Fn(usize) -> T) -> Self {
        Vector {
            values: functions::build_array(f),
        }
    }
}

impl<T, const N: usize> Clone for Vector<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl<T, const N: usize> Copy for Vector<T, N> where T: Clone + Copy {}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            values: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Display for Vector<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        for i in 0..(N - 1) {
            write!(f, "{},", i)?;
        }
        write!(f, "{})", N - 1)
    }
}

impl<T, const N: usize> Debug for Vector<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&format!("Vector<{},{}>", type_name::<T>(), N))
            .field("values", &self.values)
            .finish()
    }
}

impl<T, const N: usize> PartialEq for Vector<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T, const N: usize> Eq for Vector<T, N> where T: PartialEq + Eq {}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

/*
pub fn build(f: impl Fn(usize) -> T) -> Self {
        Vector {
            values: {
                if !core::mem::needs_drop::<T>() {
                    let mut values: MaybeUninit<[T; N]> = MaybeUninit::uninit();
                    let mut ptr_i = values.as_mut_ptr() as *mut T;
                    unsafe {
                        for i in 0..N {
                            ptr_i.write((f)(i));
                            ptr_i = ptr_i.add(1);
                        }
                    }
                    unsafe { values.assume_init() }
                } else {
                    struct UnsafeDropSliceGuard<Item> {
                        base_ptr: *mut Item,
                        initialized_count: usize,
                    }
                    impl<Item> Drop for UnsafeDropSliceGuard<Item> {
                        fn drop(self: &'_ mut Self) {
                            unsafe {
                                // # Safety
                                //
                                //   - the contract of the struct guarantees that this is sound
                                core::ptr::drop_in_place(core::slice::from_raw_parts_mut(
                                    self.base_ptr,
                                    self.initialized_count,
                                ));
                            }
                        }
                    }
                    unsafe {
                        let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
                        // pointer to array = *mut [T; N] <-> *mut T = pointer to first element
                        let mut ptr_i = array.as_mut_ptr() as *mut T;
                        let mut panic_guard = UnsafeDropSliceGuard {
                            base_ptr: ptr_i,
                            initialized_count: 0,
                        };

                        for i in 0..N {
                            // Invariant: `i` elements have already been initialized
                            panic_guard.initialized_count = i;
                            // If this panics or fails, `panic_guard` is dropped, thus
                            // dropping the elements in `base_ptr[.. i]`
                            // this cannot panic
                            // the previously uninit value is overwritten without being read or dropped
                            ptr_i.write((f)(i));
                            ptr_i = ptr_i.add(1);
                        }
                        // From now on, the code can no longer `panic!`, let's take the
                        // symbolic ownership back
                        core::mem::forget(panic_guard);
                        array.assume_init()
                    }
                }
            },
        }
    }
*/
