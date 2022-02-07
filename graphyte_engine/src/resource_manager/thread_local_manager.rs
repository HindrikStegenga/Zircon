use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct ThreadLocalResourceManager {
    resources: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl Default for ThreadLocalResourceManager {
    fn default() -> Self {
        Self {
            resources: Default::default(),
        }
    }
}

impl ThreadLocalResourceManager {
    pub fn add_resource<T: Send + 'static>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), Box::from(resource));
    }

    pub fn remove_resource<T: Send + 'static>(&mut self) -> Option<T> {
        let value = self.resources.remove(&TypeId::of::<T>())?;
        Some(*value.downcast::<T>().ok()?)
    }

    pub fn get_resource<T: Send + 'static>(&self) -> Option<&T> {
        let value = self.resources.get(&TypeId::of::<T>())?;
        Some(value.downcast_ref::<T>()?)
    }

    pub fn contains<T: Send + 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    pub fn get_resource_mut<T: Send + 'static>(&mut self) -> Option<&mut T> {
        let value = self.resources.get_mut(&TypeId::of::<T>())?;
        Some(value.downcast_mut::<T>()?)
    }
}
