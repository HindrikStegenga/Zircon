use super::error::ResourceError;
use std::sync::Arc;

pub trait UniqueResourceProvider<R: UniqueResource>: 'static {
    fn provide_resource(&mut self, info: R::ResourceRequestInfo) -> Result<R, ResourceError>;
}

pub trait UniqueResource: Sized + 'static {
    type ResourceRequestInfo;
    const IS_REMOVABLE: bool = true;
}

impl<T: UniqueResource> UniqueResource for Box<T> {
    type ResourceRequestInfo = T::ResourceRequestInfo;
}

impl<T: UniqueResource> UniqueResource for Arc<T> {
    type ResourceRequestInfo = T::ResourceRequestInfo;
}
