#![allow(unused)]
use super::header::*;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use xxhash_rust::*;
use zstd::bulk::*;

#[derive(Debug)]
pub enum AssetArchiveError {
    InvalidHeaderHash,
    HeaderDeserializationError(serde_cbor::Error),
    IO(tokio::io::Error),
}

impl std::error::Error for AssetArchiveError {}
impl std::fmt::Display for AssetArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetArchiveError::InvalidHeaderHash => f.write_str("Invalid header hash detected."),
            AssetArchiveError::IO(e) => e.fmt(f),
            AssetArchiveError::HeaderDeserializationError(e) => e.fmt(f),
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

/// MemoryLayout:
/// - magic value               - 4 bytes u32 (LE)
/// - files                     - N bytes
/// - compressed header         - N bytes (serialized using flatbuffer and compressed using zstd) (LE)
/// - decompressed header size  - 8 bytes u64 (LE)
/// - compressed header hash    - 8 bytes xxh3 hash (LE)
/// - compressed header size    - 8 bytes u64 (LE)
#[derive(Serialize, Deserialize, Hash)]
pub struct AssetArchive {
    header: ArchiveHeader,
}

impl AssetArchive {
    const MAGIC_VALUE: u32 = 0x85aadc86;

    /// Reads the magic value that is required at the start of each archive.
    pub async fn read_magic_value(
        mut reader: impl AsyncBufReadExt + AsyncSeekExt + Unpin,
    ) -> Result<bool, tokio::io::Error> {
        let mut magic_value_buffer: [u8; 4] = [0; 4];
        // Get the magic value
        reader.seek(SeekFrom::Start(0)).await?;
        reader.read_exact(&mut magic_value_buffer).await?;
        Ok(u32::from_le_bytes(magic_value_buffer) == Self::MAGIC_VALUE)
    }

    /// Writes the magic value into the writer.
    pub async fn write_magic_value(
        mut writer: impl AsyncWriteExt + Unpin,
    ) -> Result<(), tokio::io::Error> {
        // Write the magic value.
        let mut magic_value_buffer: [u8; 4] = Self::MAGIC_VALUE.to_le_bytes();
        writer.write_all(&magic_value_buffer).await?;
        Ok(())
    }

    /// Reads the header at the end of each archive.
    /// If successful, the reader is guaranteed to be positioned at the end of the compressed header block.
    /// Otherwise the reader is at an unspecified position.
    pub async fn read_header(
        mut reader: impl AsyncBufReadExt + AsyncSeekExt + Unpin,
    ) -> Result<ArchiveHeader, AssetArchiveError> {
        let mut compressed_header_size: [u8; 8] = [0; 8];
        let mut decompressed_size: [u8; 8] = [0; 8];
        let mut header_hash: [u8; 8] = [0; 8];

        // Read the decompressed header size.
        reader.seek(SeekFrom::End(-24)).await?;
        reader.read_exact(&mut decompressed_size).await?;

        // Read the hash.
        reader.seek(SeekFrom::End(-16)).await?;
        reader.read_exact(&mut header_hash).await?;

        // Read compressed header size.
        reader.seek(SeekFrom::End(-8)).await?;
        reader.read_exact(&mut compressed_header_size).await?;

        // Convert them into u64's.
        let compressed_header_size = u64::from_le_bytes(compressed_header_size);
        let decompressed_header_size = u64::from_le_bytes(decompressed_size);
        let header_hash = u64::from_le_bytes(header_hash);

        // Read the archive's compressed header.
        let mut compressed_header = vec![0u8; compressed_header_size as usize];
        reader
            .seek(SeekFrom::End(-24 - (compressed_header_size as i64)))
            .await?;
        reader.read_exact(&mut compressed_header).await?;

        // Check the hash of the compressed header.
        if header_hash != xxh3::xxh3_64(&compressed_header) {
            return Err(AssetArchiveError::InvalidHeaderHash);
        }

        // Hash is fine, so we decompress the header.
        let mut decompressed_header = vec![0u8; decompressed_header_size as usize];
        decompress_to_buffer(&compressed_header, &mut decompressed_header);

        // Headers are always saved in cbor format.
        let header = serde_cbor::de::from_slice::<ArchiveHeader>(&decompressed_header)?;

        Ok(header)
    }

    /// Writes the header to the end of the archive.
    /// This function asumes that the write position is at the end of the file block.
    pub async fn write_header(
        header: ArchiveHeader,
        mut writer: impl AsyncWriteExt + Unpin,
    ) -> Result<(), AssetArchiveError> {
        // Convert the header to packed cbor format.
        let uncompressed_header = serde_cbor::ser::to_vec_packed(&header)?;
        // Compress the header using zstd.
        let compressed_header = zstd::bulk::compress(&uncompressed_header, 0)?;
        // Hash the compressed header and convert it and the lengths to LE bytes.
        let compressed_header_hash = xxh3::xxh3_64(&compressed_header).to_le_bytes();
        let compressed_header_size = (compressed_header.len() as u64).to_le_bytes();
        let uncompressed_header_size = (uncompressed_header.len() as u64).to_le_bytes();

        // Write the header to the writer.
        writer.write_all(&compressed_header).await?;
        // Write uncompressed header size.
        writer.write_all(&uncompressed_header_size).await?;
        // Write compressed header hash.
        writer.write_all(&compressed_header_hash).await?;
        // Write compressed header size.
        writer.write_all(&compressed_header_size).await?;

        Ok(())
    }
}
