use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ResourceError {
    UnknownResourceType,
    UnknownResourceKey,
    ResourceTypeAlreadyExists,
    NoResourceProvider,
    UnremovableResource,
    ResourceProviderFailure,
}

impl Error for ResourceError {}
impl Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::UnknownResourceType => write!(f, "Unknown resource type."),
            ResourceError::UnknownResourceKey => {
                write!(f, "Unknown key for the given resource type.")
            }
            ResourceError::ResourceTypeAlreadyExists => write!(f, "Resource type already exists."),
            ResourceError::NoResourceProvider => write!(
                f,
                "There was no resource provider registered for the given resource type."
            ),
            ResourceError::UnremovableResource => {
                write!(f, "The specified resource cannot be removed.")
            }
            ResourceError::ResourceProviderFailure => {
                write!(f, "The requested resource could not be provided.")
            }
        }
    }
}
