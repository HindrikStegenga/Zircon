pub mod error;

use self::error::ResourceError;
use anymap::AnyMap;

pub trait UniqueResourceProvider<R: UniqueResource>: 'static {
    fn provide_resource(&mut self, info: R::ResourceRequestInfo) -> Result<R, ResourceError>;
}
pub trait Resource: Sized + 'static {
    type ResourceRequestInfo;
}
pub trait UniqueResource: Sized + 'static {
    type ResourceRequestInfo;
}
#[derive(Debug)]
pub struct ResourceSystem {
    unique_resources: AnyMap,
    unique_resource_providers: AnyMap,
}

impl ResourceSystem {
    /// Adds the unique resource instance into the system. Returns Err if a unique resource already existed.
    pub fn add_unique_resource<R: UniqueResource>(&mut self, resource: R) -> Result<(), R> {
        match self.unique_resources.get::<R>() {
            Some(_) => return Err(resource),
            None => self.unique_resources.insert(resource),
        };
        Ok(())
    }

    /// Removes a unique resource from the system.
    pub fn remove_unique_resource<R: UniqueResource>(&mut self) -> Result<R, ResourceError> {
        return match self.unique_resources.remove::<R>() {
            Some(resource) => Ok(resource),
            None => Err(ResourceError::UnknownResourceKey),
        };
    }

    /// Request that a specific resource is made available.
    pub fn request_unique_resource<R: UniqueResource>(
        &mut self,
        request_info: R::ResourceRequestInfo,
    ) -> Result<&mut R, ResourceError> {
        if self.unique_resources.contains::<R>() {
            return Err(ResourceError::ResourceTypeAlreadyExists);
        }

        match self
            .unique_resource_providers
            .get_mut::<Box<dyn UniqueResourceProvider<R>>>()
        {
            Some(provider) => {
                let resource = provider.provide_resource(request_info)?;
                self.unique_resources.insert(resource);
                Ok(self.unique_resources.get_mut::<R>().unwrap())
            }
            None => Err(ResourceError::NoResourceProvider),
        }
    }

    pub fn add_unique_resource_provider<R: UniqueResource, P: UniqueResourceProvider<R>>(
        &mut self,
        provider: P,
    ) -> Result<(), P> {
        if self
            .unique_resource_providers
            .contains::<Box<dyn UniqueResourceProvider<R>>>()
        {
            return Err(provider);
        }
        self.unique_resource_providers.insert(Box::new(provider));
        Ok(())
    }

    pub fn remove_unique_resource_provider<R: UniqueResource, P: UniqueResourceProvider<R>>(
        &mut self,
    ) -> Result<P, ResourceError> {
        match self.unique_resource_providers.remove() {
            Some(provider) => Ok(provider),
            None => Err(ResourceError::NoResourceProvider),
        }
    }
}
