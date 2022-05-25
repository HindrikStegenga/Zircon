use dashmap::*;
use std::any::{Any, TypeId};
use std::sync::Arc;

#[derive(Debug)]
pub struct EngineResourceManager {
    engine_resources: DashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Default for EngineResourceManager {
    fn default() -> Self {
        Self {
            engine_resources: Default::default(),
        }
    }
}

impl EngineResourceManager {
    pub fn add_resource<T: Send + Sync + 'static>(&self, resource: T) {
        self.engine_resources
            .insert(TypeId::of::<T>(), Arc::from(resource));
    }

    pub fn remove_resource<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let (_key, value) = self.engine_resources.remove(&TypeId::of::<T>())?;
        Some(value.downcast::<T>().unwrap())
    }

    pub fn get_resource<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let value = self.engine_resources.get(&TypeId::of::<T>())?;
        Some(Arc::clone(value.value()).downcast::<T>().unwrap())
    }
}
