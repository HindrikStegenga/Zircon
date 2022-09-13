#![allow(unused)]
use super::{error::*, header::*};
use crate::formats::*;
use crate::AssetArchiveError::InvalidMagicValue;
use crate::AssetIdentifier;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;
use std::path::Path;
use tokio::fs::{read_dir, File};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, SeekFrom};
use tokio::sync::*;
use xxhash_rust::*;
use zstd::bulk::*;

/// MemoryLayout:
/// - magic value               - 4 bytes u32 (LE)
/// - files                     - N bytes
/// - compressed header         - N bytes (serialized using flatbuffer and compressed using zstd) (LE)
/// - decompressed header size  - 8 bytes u64 (LE)
/// - compressed header hash    - 8 bytes xxh3 hash (LE)
/// - compressed header size    - 8 bytes u64 (LE)
pub struct AssetArchive<R: AsyncReadExt + AsyncSeekExt + Unpin = File> {
    header: ArchiveHeader,
    reader: Mutex<BufReader<R>>,
}

impl<R: AsyncReadExt + AsyncSeekExt + Unpin> AssetArchive<R> {
    pub const fn header(&self) -> &ArchiveHeader {
        &self.header
    }

    pub async fn read_asset_into<'a, 'b>(
        &'a self,
        file_header_offset: usize,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], AssetArchiveError> {
        return if let Some(file_header) = self.header.files().get(file_header_offset) {
            let mut guard = self.reader.lock().await;
            read_file_into_buffer(file_header, guard.deref_mut(), buffer).await
        } else {
            Err(AssetArchiveError::UnknownAssetIdentifier)
        };
    }

    pub async fn load_from_readable(readable: R) -> Result<AssetArchive<R>, AssetArchiveError> {
        let mut buf_reader = BufReader::new(readable);
        if !read_magic_value(&mut buf_reader).await? {
            return Err(AssetArchiveError::InvalidMagicValue);
        }
        // Read header
        let header = read_header(&mut buf_reader).await?;

        Ok(Self {
            header,
            reader: tokio::sync::Mutex::new(buf_reader),
        })
    }
}

impl AssetArchive {
    pub async fn load_from_file(path: impl AsRef<Path>) -> Result<AssetArchive, AssetArchiveError> {
        let file = File::open(path).await?;
        Self::load_from_readable(file).await
    }

    pub async fn load_from_directory(
        path: impl AsRef<Path>,
        extension: impl AsRef<str>,
    ) -> Result<Vec<AssetArchive>, AssetArchiveError> {
        let mut loaded_archives = vec![];
        let mut dir_entry = read_dir(path).await?;
        while let Some(entry) = dir_entry.next_entry().await? {
            if !entry.file_type().await?.is_file() {
                continue;
            }
            if let Some(file_ext) = entry.path().extension().map(|p| p.to_str()).flatten() {
                if file_ext == extension.as_ref() {
                    let archive = Self::load_from_file(entry.path()).await?;
                    loaded_archives.push(archive);
                }
            } else {
                continue;
            }
        }
        Ok(loaded_archives)
    }
}

const MAGIC_VALUE: u32 = 0x85aadc86;

/// Reads the magic value that is required at the start of each archive.
pub async fn read_magic_value(
    mut reader: impl AsyncReadExt + AsyncSeekExt + Unpin,
) -> Result<bool, tokio::io::Error> {
    let mut magic_value_buffer: [u8; 4] = [0; 4];
    // Get the magic value
    reader.seek(SeekFrom::Start(0)).await?;
    reader.read_exact(&mut magic_value_buffer).await?;
    Ok(u32::from_le_bytes(magic_value_buffer) == MAGIC_VALUE)
}

/// Writes the magic value into the writer.
pub async fn write_magic_value(
    mut writer: impl AsyncWriteExt + Unpin,
) -> Result<(), tokio::io::Error> {
    // Write the magic value.
    let mut magic_value_buffer: [u8; 4] = MAGIC_VALUE.to_le_bytes();
    writer.write_all(&magic_value_buffer).await?;
    Ok(())
}

/// Reads the header at the end of each archive.
/// If successful, the reader is guaranteed to be positioned at the end of the compressed header block.
/// Otherwise the reader is at an unspecified position.
pub async fn read_header(
    mut reader: impl AsyncReadExt + AsyncSeekExt + Unpin,
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

/// Writes the file from the reader into the provided buffer.
/// Will only write up to `file_header.byte_count()` bytes.
pub async fn read_file_into_buffer<'a, 'b>(
    file_header: &'a FileHeader,
    mut reader: impl AsyncBufReadExt + AsyncSeekExt + Unpin,
    buffer: &'b mut [u8],
) -> Result<&'b mut [u8], AssetArchiveError> {
    if (buffer.len() as u32) < file_header.byte_count() {
        return Err(AssetArchiveError::BufferTooSmall);
    }
    // Set the reader to the appropriate offset.
    reader
        .seek(SeekFrom::Start(file_header.offset() as u64))
        .await?;
    return match file_header.compressed_format() {
        ArchiveCompressionFormat::None => {
            let read_bytes = reader
                .read_exact(&mut buffer[0..(file_header.byte_count() as usize)])
                .await?;
            Ok(&mut buffer[0..read_bytes])
        }
        ArchiveCompressionFormat::ZSTD => {
            use async_compression::tokio::bufread::ZstdDecoder;
            let mut decoder = ZstdDecoder::new(&mut reader);
            let read_bytes = decoder
                .read_exact(&mut buffer[0..(file_header.byte_count() as usize)])
                .await?;
            Ok(&mut buffer[0..read_bytes])
        }
    };
}
