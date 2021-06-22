use super::HandleType;
use std::marker::PhantomData;
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

/// Opaque handle type storing both the actual integer value as well as versioning information.
pub struct VersionedHandle<T, K = u32, G = K>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
    pub value: K,
    pub version: G,
    /// Satisfy type check and drop check.
    _phantom: PhantomData<fn(*const T)>,
}

impl<T, K, G> VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
    pub fn into<R: ?Sized>(self) -> VersionedHandle<R, K, G> {
        VersionedHandle {
            value: self.value,
            version: self.version,
            _phantom: PhantomData,
        }
    }
}

impl<T, K, G> Clone for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            version: self.version,
            _phantom: PhantomData,
        }
    }
}

impl<T, K> Hash for VersionedHandle<T, K>
where
    T: ?Sized,
    K: HandleType,
{
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(h);
        self.version.hash(h);
    }
}

impl<T, K, G> Debug for VersionedHandle<T, K, G>
where
    K: HandleType + Debug,
    G: HandleType + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "GenerationalHandle<{}>",
            std::any::type_name::<T>()
        ))
        .field("value", &self.value)
        .field("generation", &self.version)
        .finish()
    }
}

impl<T, K, G> Copy for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
}

impl<T, K, G> From<(K, G)> for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
    fn from(value: (K, G)) -> Self {
        let (v, g) = value;
        Self {
            value: v,
            version: g,
            _phantom: PhantomData,
        }
    }
}

impl<T, K, G> Eq for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
}
impl<T, K, G> PartialEq for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType,
    G: HandleType,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.version == other.version
    }
}

unsafe impl<T, K, G> Send for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType + Send,
    G: HandleType + Send,
{
}
unsafe impl<T, K, G> Sync for VersionedHandle<T, K, G>
where
    T: ?Sized,
    K: HandleType + Sync,
    G: HandleType + Sync,
{
}
