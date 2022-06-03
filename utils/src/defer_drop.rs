use std::{
    any::Any,
    sync::{Arc, Weak},
};

/// Object intended to defer dropping a resource.
/// It's primary usecase is in message handling, to ensure a resource still
/// exists until all messages that it is destroyed are processed.
#[derive(Debug)]
pub struct DeferDrop {
    _resource: Arc<dyn Any + Send>,
}

impl Clone for DeferDrop {
    fn clone(&self) -> Self {
        Self {
            _resource: Arc::clone(&self._resource),
        }
    }
}

impl DeferDrop {
    pub fn new<T: Send + 'static>(resource: T) -> Self {
        Self {
            _resource: Arc::from(resource),
        }
    }

    pub fn is_dropped(&self) -> bool {
        Arc::strong_count(&self._resource) == 0
    }

    pub fn weak(&self) -> WeakDeferDrop {
        WeakDeferDrop {
            _resource: Arc::downgrade(&self._resource),
        }
    }
}
// We can safely implement send since the object is immutable. (In principle even sync)
unsafe impl Send for DeferDrop {}

/// Weak version of [`DeferDrop`];
#[derive(Debug)]
pub struct WeakDeferDrop {
    _resource: Weak<dyn Any + Send>,
}

impl WeakDeferDrop {
    pub fn is_dropped(&self) -> bool {
        Weak::strong_count(&self._resource) == 0
    }
}

// We can safely implement send since the object is immutable. (In principle even sync)
unsafe impl Send for WeakDeferDrop {}

impl Clone for WeakDeferDrop {
    fn clone(&self) -> Self {
        Self {
            _resource: Weak::clone(&self._resource),
        }
    }
}
