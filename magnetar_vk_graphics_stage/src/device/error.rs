use erupt::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum VkDeviceError {
    LoaderError(LoaderError),
    VkResultFailure(vk::Result),
}
impl std::error::Error for VkDeviceError {}
impl Display for VkDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VkDeviceError::LoaderError(e) => e.fmt(f),
            VkDeviceError::VkResultFailure(e) => e.fmt(f),
        }
    }
}
impl From<LoaderError> for VkDeviceError {
    fn from(e: LoaderError) -> Self {
        Self::LoaderError(e)
    }
}
impl From<vk::Result> for VkDeviceError {
    fn from(e: vk::Result) -> Self {
        Self::VkResultFailure(e)
    }
}
