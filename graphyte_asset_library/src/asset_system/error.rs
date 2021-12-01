use std::{
    error::Error,
    fmt::{write, Display},
    sync::{PoisonError, RwLockReadGuard},
};

use crate::{
    archive::AssetArchiveError,
    vfs::{error::VfsError, VirtualFileSystem},
};

#[derive(Debug)]
pub enum AssetSystemError {
    Vfs(VfsError),
    Io(std::io::Error),
    UnknownAssetFormat,
    Other(Box<dyn Error>),
    Archive(AssetArchiveError),
    PoisonError,
    NotMounted,
}
impl Error for AssetSystemError {}
impl Display for AssetSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetSystemError::Vfs(e) => e.fmt(f),
            AssetSystemError::Io(e) => e.fmt(f),
            AssetSystemError::Other(e) => e.fmt(f),
            AssetSystemError::Archive(e) => e.fmt(f),
            AssetSystemError::UnknownAssetFormat => write!(f, "Unknown asset format."),
            AssetSystemError::PoisonError => write!(
                f,
                "Poisoning occured! A thread has paniced and killed the asset system."
            ),
            AssetSystemError::NotMounted => write!(f, "Mountpoint was not mounted."),
        }
    }
}
impl From<std::io::Error> for AssetSystemError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<VfsError> for AssetSystemError {
    fn from(e: VfsError) -> Self {
        Self::Vfs(e)
    }
}
impl From<Box<dyn Error>> for AssetSystemError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::Other(e)
    }
}
impl From<AssetArchiveError> for AssetSystemError {
    fn from(e: AssetArchiveError) -> Self {
        AssetSystemError::Archive(e)
    }
}
