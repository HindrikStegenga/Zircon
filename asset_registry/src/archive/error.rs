#[derive(Debug)]
pub enum AssetArchiveError {
    InvalidMagicValue,
    InvalidHeaderHash,
    UnknownAssetIdentifier,
    HeaderDeserializationError(serde_cbor::Error),
    InputOutput(tokio::io::Error),
    BufferTooSmall,
}

impl std::error::Error for AssetArchiveError {}
impl std::fmt::Display for AssetArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetArchiveError::InvalidHeaderHash => f.write_str("Invalid header hash detected."),
            AssetArchiveError::InputOutput(e) => e.fmt(f),
            AssetArchiveError::HeaderDeserializationError(e) => e.fmt(f),
            AssetArchiveError::BufferTooSmall => f.write_str("The provided buffer was too small."),
            AssetArchiveError::InvalidMagicValue => f.write_str("Invalid magic value."),
            AssetArchiveError::UnknownAssetIdentifier => f.write_str("Unknown asset identifier."),
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
        Self::InputOutput(e)
    }
}
