use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetArchiveHeader {
    assets: Vec<AssetArchiveFileHeader>,
}

impl AssetArchiveHeader {
    pub fn new(assets: Vec<AssetArchiveFileHeader>) -> Self {
        Self { assets }
    }

    /// Get a reference to the asset archive header's assets.
    pub fn assets(&self) -> &[AssetArchiveFileHeader] {
        self.assets.as_slice()
    }

    /// Get a mutable reference to the asset archive header's assets.
    pub fn assets_mut(&mut self) -> &mut Vec<AssetArchiveFileHeader> {
        &mut self.assets
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetArchiveFileHeader {
    offset: u64,
    compressed_size: u64,
    uncompressed_size: u64,
    compression_format: AssetArchiveCompressionFormat,
}

impl AssetArchiveFileHeader {
    pub fn new(
        offset: u64,
        compressed_size: u64,
        uncompressed_size: u64,
        compression_format: AssetArchiveCompressionFormat,
    ) -> Self {
        Self {
            offset,
            compressed_size,
            uncompressed_size,
            compression_format,
        }
    }

    /// Get a reference to the asset archive file header's offset.
    pub fn offset(&self) -> &u64 {
        &self.offset
    }

    /// Get a reference to the asset archive file header's compressed size.
    pub fn compressed_size(&self) -> &u64 {
        &self.compressed_size
    }

    /// Get a reference to the asset archive file header's uncompressed size.
    pub fn uncompressed_size(&self) -> &u64 {
        &self.uncompressed_size
    }

    /// Get a reference to the asset archive file header's compression format.
    pub fn compression_format(&self) -> &AssetArchiveCompressionFormat {
        &self.compression_format
    }
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AssetArchiveCompressionFormat {
    None = 0,
    LZ4 = 1,
}
