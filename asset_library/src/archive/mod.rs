use std::{
    fs::*,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

pub mod builder;
pub mod error;
pub mod header;

pub use builder::*;
pub use error::*;
pub use header::*;
use lz4_flex::decompress_into;

// AssetArchive is a type storing multiple potentially compressed assets into a single archive.
#[derive(Debug)]
pub struct AssetArchive {
    header: AssetArchiveHeader,
    path: PathBuf,
}

impl AssetArchive {
    /// Reads an asset archive from a file. Only succeeds in case the provided file can be interpreted as an archive.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<AssetArchive, AssetArchiveError> {
        let file = std::fs::File::open(path.as_ref())?;
        let reader = std::io::BufReader::new(&file);
        let header = Self::read_header(reader)?;
        Ok(Self {
            header,
            path: PathBuf::from(path.as_ref()),
        })
    }

    fn read_header(
        mut reader: std::io::BufReader<&std::fs::File>,
    ) -> Result<AssetArchiveHeader, AssetArchiveError> {
        // last 8 bytes are compressed header size in LE byte order
        let mut compressed_header_size: [u8; 8] = [0; 8];
        reader.seek(SeekFrom::End(-8))?;
        reader.read_exact(&mut compressed_header_size)?;
        let compressed_header_size = u64::from_le_bytes(compressed_header_size);
        // second 8 last bytes are uncompressed header size in LE byte order
        let mut uncompressed_header_size: [u8; 8] = [0; 8];
        reader.seek(SeekFrom::End(-16))?;
        reader.read_exact(&mut uncompressed_header_size)?;
        let uncompressed_header_size = u64::from_le_bytes(uncompressed_header_size);
        // The next `compressed_header_size` bytes are the bytes of the CBOR encoded header.
        let mut compressed_header = Vec::with_capacity(compressed_header_size as usize);
        unsafe { compressed_header.set_len(compressed_header_size as usize) };
        reader.seek(SeekFrom::End(-16 - compressed_header_size as i64))?;
        reader.read_exact(&mut compressed_header)?;
        let uncompressed_header =
            zstd::bulk::decompress(&compressed_header, uncompressed_header_size as usize)?;
        let header = serde_cbor::from_slice::<AssetArchiveHeader>(&uncompressed_header)?;
        Ok(header)
    }

    /// Get a reference to the asset archive's header.
    pub fn header(&self) -> &AssetArchiveHeader {
        &self.header
    }

    // Reads a blob from the current archive using the provided header.
    pub fn read_file(
        path: impl AsRef<Path>,
        header: &AssetArchiveFileHeader,
    ) -> Result<Vec<u8>, AssetArchiveError> {
        let mut buf = Vec::new();
        Self::read_file_into(path, header, &mut buf)?;
        Ok(buf)
    }

    // Reads a blob from the current archive using the provided header.
    pub fn read_file_from(
        &self,
        header: &AssetArchiveFileHeader,
    ) -> Result<Vec<u8>, AssetArchiveError> {
        let mut buf = Vec::new();
        Self::read_file_into(&self.path, header, &mut buf)?;
        Ok(buf)
    }

    pub fn read_file_into(
        path: impl AsRef<Path>,
        header: &AssetArchiveFileHeader,
        mut buffer: &mut Vec<u8>,
    ) -> Result<(), AssetArchiveError> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(*header.offset()))?;

        match header.compression_format() {
            AssetArchiveCompressionFormat::None => {
                buffer.resize(*header.compressed_size() as usize, 0);
                reader.read_exact(&mut buffer)?;
                Ok(())
            }
            AssetArchiveCompressionFormat::LZ4 => {
                let mut temp_buffer = Vec::with_capacity(*header.compressed_size() as usize);
                unsafe { temp_buffer.set_len(*header.compressed_size() as usize) };
                reader.read_exact(&mut temp_buffer)?;
                buffer.resize(*header.uncompressed_size() as usize, 0);
                lz4_flex::decompress_into(&temp_buffer, &mut buffer)?;
                Ok(())
            }
            AssetArchiveCompressionFormat::ZSTD => {
                let mut temp_buffer = Vec::with_capacity(*header.compressed_size() as usize);
                unsafe { temp_buffer.set_len(*header.compressed_size() as usize) };
                reader.read_exact(&mut temp_buffer)?;
                buffer.resize(*header.uncompressed_size() as usize, 0);
                zstd::bulk::decompress_to_buffer(&temp_buffer, &mut buffer)?;
                Ok(())
            }
        }
    }

    // pub async fn async_read_file_into(
    //     path: impl AsRef<Path>,
    //     header: &AssetArchiveFileHeader,
    //     mut buffer: &mut Vec<u8>,
    // ) -> Result<(), AssetArchiveError> {
    //     use utils::smol::{fs::*, io::*};
    //     let file = File::open(path.as_ref()).await?;
    //     let mut reader = BufReader::new(file);
    //     reader.seek(SeekFrom::Start(*header.offset())).await?;

    //     match header.compression_format() {
    //         AssetArchiveCompressionFormat::None => {
    //             buffer.resize(*header.compressed_size() as usize, 0);
    //             reader.read_exact(&mut buffer).await?;
    //             Ok(())
    //         }
    //         AssetArchiveCompressionFormat::LZ4 => {
    //             let mut temp_buffer = Vec::with_capacity(*header.compressed_size() as usize);
    //             unsafe { temp_buffer.set_len(*header.compressed_size() as usize) };
    //             reader.read_exact(&mut temp_buffer).await?;
    //             buffer.resize(*header.uncompressed_size() as usize, 0);
    //             decompress_into(&temp_buffer, &mut buffer)?;
    //             Ok(())
    //         }
    //         AssetArchiveCompressionFormat::ZSTD => {
    //             let mut temp_buffer = Vec::with_capacity(*header.compressed_size() as usize);
    //             unsafe { temp_buffer.set_len(*header.compressed_size() as usize) };
    //             reader.read_exact(&mut temp_buffer).await?;
    //             buffer.resize(*header.uncompressed_size() as usize, 0);
    //             zstd::bulk::decompress_to_buffer(&temp_buffer, &mut buffer)?;
    //             Ok(())
    //         }
    //     }
    // }

    /// Get a reference to the asset archive's path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
