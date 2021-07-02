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

// AssetArchive is a type storing multiple potentially compressed assets into a single archive.
#[derive(Debug)]
pub struct AssetArchive {
    header: AssetArchiveHeader,
    path: PathBuf,
}

impl AssetArchive {
    /// Reads an asset archive from a file. Only succeeds in case the provided file can be interpreted as an archive.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<AssetArchive, AssetArchiveError> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(&file);
        let header = Self::read_header(reader)?;
        Ok(Self {
            header,
            path: PathBuf::from(path.as_ref()),
        })
    }

    fn read_header(mut reader: BufReader<&File>) -> Result<AssetArchiveHeader, AssetArchiveError> {
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
            lz4_flex::decompress(&compressed_header, uncompressed_header_size as usize)?;
        let header = serde_cbor::from_slice::<AssetArchiveHeader>(&uncompressed_header)?;
        Ok(header)
    }

    /// Get a reference to the asset archive's header.
    pub fn header(&self) -> &AssetArchiveHeader {
        &self.header
    }

    // Reads a blob from the current archive using the provided header.
    pub fn read_blob(&self, header: &AssetArchiveFileHeader) -> Result<Vec<u8>, AssetArchiveError> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(*header.offset()))?;
        let mut buffer = Vec::with_capacity(*header.compressed_size() as usize);
        unsafe { buffer.set_len(*header.compressed_size() as usize) };
        reader.read_exact(&mut buffer)?;
        if *header.compression_format() == AssetArchiveCompressionFormat::None {
            Ok(buffer)
        } else {
            use lz4_flex::decompress;
            let decompressed = decompress(&buffer, *header.uncompressed_size() as usize)?;
            Ok(decompressed)
        }
    }
}
