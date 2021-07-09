use erupt::{utils::loading::EntryLoaderError, *};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VkGraphicsSystemError {
    EntryLoaderError(EntryLoaderError),
    LoaderError(LoaderError),
}

impl Error for VkGraphicsSystemError {}

impl Display for VkGraphicsSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VkGraphicsSystemError::EntryLoaderError(e) => e.fmt(f),
            VkGraphicsSystemError::LoaderError(e) => e.fmt(f),
        }
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
