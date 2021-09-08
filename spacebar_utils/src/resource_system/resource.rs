use super::error::ResourceError;

pub trait Resource: Sized + 'static {
    type ResourceRequestInfo;
    const IS_REMOVABLE: bool = true;
}

pub trait ResourceProvider<R: Resource>: 'static {
    fn provide_resource(&mut self, info: R::ResourceRequestInfo) -> Result<R, ResourceError>;
}
