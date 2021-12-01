use lz4_flex::block::{CompressError, DecompressError};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AssetArchiveError {
    Io(std::io::Error),
    Compress(CompressError),
    Decompress(DecompressError),
    DeserializeError(serde_cbor::Error),
    InvalidMountPoint,
}

impl Display for AssetArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::Compress(e) => write!(f, "{}", e),
            Self::Decompress(e) => write!(f, "{}", e),
            Self::DeserializeError(e) => write!(f, "{}", e),
            Self::InvalidMountPoint => write!(f, "Invalid mount point."),
        }
    }
}
impl Error for AssetArchiveError {}

impl From<std::io::Error> for AssetArchiveError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<CompressError> for AssetArchiveError {
    fn from(err: CompressError) -> Self {
        Self::Compress(err)
    }
}

impl From<DecompressError> for AssetArchiveError {
    fn from(err: DecompressError) -> Self {
        Self::Decompress(err)
    }
}
impl From<serde_cbor::Error> for AssetArchiveError {
    fn from(err: serde_cbor::Error) -> Self {
        Self::DeserializeError(err)
    }
}
