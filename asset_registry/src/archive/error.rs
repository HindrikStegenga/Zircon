#[derive(Debug)]
pub enum AssetArchiveError {
    InvalidHeaderHash,
    HeaderDeserializationError(serde_cbor::Error),
    IO(tokio::io::Error),
    BufferTooSmall,
}

impl std::error::Error for AssetArchiveError {}
impl std::fmt::Display for AssetArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetArchiveError::InvalidHeaderHash => f.write_str("Invalid header hash detected."),
            AssetArchiveError::IO(e) => e.fmt(f),
            AssetArchiveError::HeaderDeserializationError(e) => e.fmt(f),
            AssetArchiveError::BufferTooSmall => f.write_str("The provided buffer was too small."),
        }
    }
}

impl From<serde_cbor::Error> for AssetArchiveError {
    fn from(e: serde_cbor::Error) -> Self {
        Self::HeaderDeserializationError(e)
    }
}
impl From<tokio::io::Error> for AssetArchiveError {
    fn from(e: tokio::io::Error) -> Self {
        Self::IO(e)
    }
}
