use core::{marker::PhantomData, mem::MaybeUninit};

use super::Vector;
use alloc::format;
use serde::{
    de::{self, Visitor},
    ser::SerializeTuple,
    Deserialize, Serialize,
};

struct VectorTNVisitor<T, const N: usize> {
    marker: PhantomData<fn() -> [T; N]>,
}

impl<T, const N: usize> VectorTNVisitor<T, N> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, T, const N: usize> Visitor<'de> for VectorTNVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = [T; N];

    fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        formatter.write_str(&format!(
            "Expecting [{};{}]",
            core::any::type_name::<T>(),
            N
        ))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // Safety: `assume_init` is sound because the type we are claiming to have
        // initialized here is a bunch of `MaybeUninit`s, which do not require
        // initialization.
        let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        // Iterate over the array and fill the elemenets with the ones obtained from
        // `seq`.
        let mut place_iter = arr.iter_mut();
        let mut cnt_filled = 0;
        let err = loop {
            match (seq.next_element(), place_iter.next()) {
                (Ok(Some(val)), Some(place)) => *place = MaybeUninit::new(val),
                // no error, we're done
                (Ok(None), None) => break None,
                // error from serde, propagate it
                (Err(e), _) => break Some(e),
                // lengths do not match, report invalid_length
                (Ok(None), Some(_)) | (Ok(Some(_)), None) => {
                    break Some(de::Error::invalid_length(cnt_filled, &self))
                }
            }
            cnt_filled += 1;
        };
        if let Some(err) = err {
            if core::mem::needs_drop::<T>() {
                for elem in core::array::IntoIter::new(arr).take(cnt_filled) {
                    // Safety: `assume_init()` is sound because we did initialize CNT_FILLED
                    // elements. We call it to drop the deserialized values.
                    unsafe {
                        elem.assume_init();
                    }
                }
            }
            return Err(err);
        }

        let ret = unsafe { core::mem::transmute_copy(&arr) };
        core::mem::forget(arr);

        Ok(ret)
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for Vector<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_tuple_struct("values", N, VectorTNVisitor::new())
            .map(|a| Vector::<T, N>::from(a))
    }
}

impl<T: Serialize, const N: usize> Serialize for Vector<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tup = serializer.serialize_tuple(N)?;
        for e in &self.values {
            tup.serialize_element(e)?;
        }
        tup.end()
    }
}
