use crate::{resource_system::*, slot_maps::VersionedSlotMap};
use anymap::{any::Any, Map};

#[derive(Debug)]
pub struct SendableResourceSystem {
    unique_resources: Map<dyn Any + Send>,
    unique_resource_providers: Map<dyn Any + Send>,
    resources: Map<dyn Any + Send>,
    resource_providers: Map<dyn Any + Send>,
}

impl Default for SendableResourceSystem {
    fn default() -> Self {
        Self {
            unique_resource_providers: Map::new(),
            unique_resources: Map::new(),
            resources: Map::new(),
            resource_providers: Map::new(),
        }
    }
}

impl SendableResourceSystem {
    /// Adds the unique resource instance into the system. Returns Err if a unique resource already existed.
    pub fn add_unique_resource<R: UniqueResource + Send>(&mut self, resource: R) -> Result<(), R> {
        match self.unique_resources.get::<R>() {
            Some(_) => return Err(resource),
            None => self.unique_resources.insert(resource),
        };
        Ok(())
    }

    /// Removes a unique resource from the system.
    pub fn remove_unique_resource<R: UniqueResource + Send>(&mut self) -> Result<R, ResourceError> {
        if !R::IS_REMOVABLE {
            return Err(ResourceError::UnremovableResource);
        }
        return match self.unique_resources.remove::<R>() {
            Some(resource) => Ok(resource),
            None => Err(ResourceError::UnknownResourceKey),
        };
    }

    /// Request that a specific resource is made available.
    pub fn request_unique_resource<R: UniqueResource + Send>(
        &mut self,
        request_info: R::ResourceRequestInfo,
    ) -> Result<&mut R, ResourceError> {
        if self.unique_resources.contains::<R>() {
            return Err(ResourceError::ResourceTypeAlreadyExists);
        }

        match self
            .unique_resource_providers
            .get_mut::<Box<dyn UniqueResourceProvider<R> + Send>>()
        {
            Some(provider) => {
                let resource = provider.provide_resource(request_info)?;
                self.unique_resources.insert(resource);
                Ok(self.unique_resources.get_mut::<R>().unwrap())
            }
            None => Err(ResourceError::NoResourceProvider),
        }
    }

    pub fn add_unique_resource_provider<
        R: UniqueResource + Send,
        P: UniqueResourceProvider<R> + Send,
    >(
        &mut self,
        provider: P,
    ) -> Result<(), P> {
        if self
            .unique_resource_providers
            .contains::<Box<dyn UniqueResourceProvider<R> + Send>>()
        {
            return Err(provider);
        }
        self.unique_resource_providers.insert(Box::new(provider));
        Ok(())
    }

    pub fn remove_unique_resource_provider<
        R: UniqueResource + Send,
        P: UniqueResourceProvider<R> + Send,
    >(
        &mut self,
    ) -> Result<P, ResourceError> {
        match self.unique_resource_providers.remove() {
            Some(provider) => Ok(provider),
            None => Err(ResourceError::NoResourceProvider),
        }
    }

    pub fn get_unique_resource<R: UniqueResource + Send>(&self) -> Option<&R> {
        self.unique_resources.get::<R>()
    }
    pub fn get_unique_resource_mut<R: UniqueResource + Send>(&mut self) -> Option<&mut R> {
        self.unique_resources.get_mut::<R>()
    }

    /// Adds the resource instance into the system.
    pub fn add_resource<R: Resource + Send>(
        &mut self,
        resource: R,
    ) -> Result<ResourceHandle<R>, R> {
        match self.resources.get_mut::<VersionedSlotMap<R, u16, u8>>() {
            Some(slot_map) => {
                let handle = slot_map.add(resource);
                Ok(handle)
            }
            None => {
                let mut slot_map = VersionedSlotMap::<R, u16, u8>::new();
                let handle = slot_map.add(resource);
                self.resources.insert(slot_map);
                Ok(handle)
            }
        }
    }

    /// Removes a resource from the system.
    pub fn remove_resource<R: Resource + Send>(
        &mut self,
        handle: ResourceHandle<R>,
    ) -> Result<R, ResourceError> {
        if !R::IS_REMOVABLE {
            return Err(ResourceError::UnremovableResource);
        }
        match self.resources.get_mut::<VersionedSlotMap<R, u16, u8>>() {
            Some(slot_map) => {
                if let Some(resource) = slot_map.remove(handle) {
                    Ok(resource)
                } else {
                    Err(ResourceError::UnknownResourceKey)
                }
            }
            None => Err(ResourceError::UnknownResourceType),
        }
    }

    /// Request that a new specific resource instance is made available.
    pub fn request_resource<R: Resource + Send>(
        &mut self,
        request_info: R::ResourceRequestInfo,
    ) -> Result<ResourceHandle<R>, ResourceError> {
        match self
            .resource_providers
            .get_mut::<Box<dyn ResourceProvider<R> + Send>>()
        {
            Some(provider) => {
                let slot_map = match self.resources.get_mut::<VersionedSlotMap<R, u16, u8>>() {
                    Some(v) => v,
                    None => {
                        let map = VersionedSlotMap::<R, u16, u8>::new();
                        self.resources.insert(map);
                        self.resources
                            .get_mut::<VersionedSlotMap<R, u16, u8>>()
                            .unwrap()
                    }
                };

                let resource = provider.provide_resource(request_info)?;
                let handle = slot_map.add(resource);
                Ok(handle)
            }
            None => Err(ResourceError::NoResourceProvider),
        }
    }

    pub fn add_resource_provider<R: Resource + Send, P: ResourceProvider<R> + Send>(
        &mut self,
        provider: P,
    ) -> Result<(), P> {
        if self
            .resource_providers
            .contains::<Box<dyn ResourceProvider<R> + Send>>()
        {
            return Err(provider);
        }
        self.resource_providers.insert(Box::new(provider));
        Ok(())
    }

    pub fn remove_resource_provider<R: Resource + Send, P: ResourceProvider<R> + Send>(
        &mut self,
    ) -> Result<P, ResourceError> {
        match self.resource_providers.remove() {
            Some(provider) => Ok(provider),
            None => Err(ResourceError::NoResourceProvider),
        }
    }

    pub fn get_resource<R: Resource + Send>(&self, handle: ResourceHandle<R>) -> Option<&R> {
        match self.resources.get::<VersionedSlotMap<R, u16, u8>>() {
            Some(slot_map) => slot_map.get(handle),
            None => None,
        }
    }
    pub fn get_resource_mut<R: Resource + Send>(
        &mut self,
        handle: ResourceHandle<R>,
    ) -> Option<&mut R> {
        match self.resources.get_mut::<VersionedSlotMap<R, u16, u8>>() {
            Some(slot_map) => slot_map.get_mut(handle),
            None => None,
        }
    }
}
