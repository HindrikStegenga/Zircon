use super::HandleType;
use serde::{Deserialize, Serialize};
use std::{fmt::*, hash::Hash};
use std::{hash::Hasher, marker::PhantomData};

/// Opaque handle type represented using integer values internally.
#[derive(Serialize, Deserialize)]
pub struct Handle<T, K = u32>
where
    T: Sized,
    K: HandleType,
{
    #[serde(deserialize_with = "K::deserialize")]
    pub value: K,
    /// Satisfy type check and drop check.
    #[serde(skip)]
    _phantom: PhantomData<*const T>,
}

impl<T, K> Clone for Handle<T, K>
where
    T: Sized,
    K: HandleType,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantom: PhantomData,
        }
    }
}

impl<T, K> Debug for Handle<T, K>
where
    K: HandleType + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Handle<{}>", std::any::type_name::<T>()))
            .field("value", &self.value)
            .finish()
    }
}

impl<T, K> Copy for Handle<T, K> where K: HandleType {}

impl<T, K> From<K> for Handle<T, K>
where
    K: HandleType,
{
    fn from(value: K) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

impl<T, K> Eq for Handle<T, K> where K: HandleType {}
impl<T, K> PartialEq for Handle<T, K>
where
    K: HandleType,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, K> Hash for Handle<T, K>
where
    K: HandleType,
{
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(h);
    }
}

unsafe impl<T, K> Send for Handle<T, K> where K: HandleType + Send {}
unsafe impl<T, K> Sync for Handle<T, K> where K: HandleType + Sync {}
