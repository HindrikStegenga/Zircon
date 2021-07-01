use lz4_flex::block::{CompressError, DecompressError};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AssetArchiveError {
    IO(std::io::Error),
    Compress(CompressError),
    Decompress(DecompressError),
    DeserializeError(serde_cbor::Error),
}

impl Display for AssetArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetArchiveError::IO(e) => write!(f, "{}", e),
            AssetArchiveError::Compress(e) => write!(f, "{}", e),
            AssetArchiveError::Decompress(e) => write!(f, "{}", e),
            AssetArchiveError::DeserializeError(e) => write!(f, "{}", e),
        }
    }
}
impl Error for AssetArchiveError {}

impl From<std::io::Error> for AssetArchiveError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
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
