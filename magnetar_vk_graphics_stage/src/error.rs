use erupt::{utils::loading::EntryLoaderError, *};
use std::{error::Error, fmt::Display};

use crate::device::{DeviceConfiguration, DeviceConfigurationError};

#[derive(Debug)]
pub enum VkGraphicsSystemError {
    EntryLoaderError(EntryLoaderError),
    LoaderError(LoaderError),
    VulkanError(erupt::vk::Result),
    NoSuitableDevicesError,
    DeviceConfigurationError(DeviceConfigurationError),
}

impl Error for VkGraphicsSystemError {}

impl Display for VkGraphicsSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VkGraphicsSystemError::EntryLoaderError(e) => e.fmt(f),
            VkGraphicsSystemError::LoaderError(e) => e.fmt(f),
            VkGraphicsSystemError::VulkanError(e) => e.fmt(f),
            VkGraphicsSystemError::DeviceConfigurationError(e) => e.fmt(f),
            VkGraphicsSystemError::NoSuitableDevicesError => {
                write!(f, "No suitable Vulkan devices found.")
            }
        }
    }
}

impl From<DeviceConfigurationError> for VkGraphicsSystemError {
    fn from(e: DeviceConfigurationError) -> Self {
        Self::DeviceConfigurationError(e)
    }
}

impl From<erupt::vk::Result> for VkGraphicsSystemError {
    fn from(e: erupt::vk::Result) -> Self {
        Self::VulkanError(e)
    }
}

impl From<EntryLoaderError> for VkGraphicsSystemError {
    fn from(e: EntryLoaderError) -> Self {
        Self::EntryLoaderError(e)
    }
}
impl From<LoaderError> for VkGraphicsSystemError {
    fn from(e: LoaderError) -> Self {
        Self::LoaderError(e)
    }
}
