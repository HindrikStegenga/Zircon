use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetArchiveHeader {
    mount_points: Vec<AssetArchiveMountPointHeader>,
}

impl AssetArchiveHeader {
    pub fn new(mount_points: Vec<AssetArchiveMountPointHeader>) -> Self {
        Self { mount_points }
    }

    /// Get a reference to the asset archive header's mount points.
    pub fn mount_points(&self) -> &[AssetArchiveMountPointHeader] {
        self.mount_points.as_slice()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetArchiveMountPointHeader {
    version: u64,
    mount_point: String,
    assets: Vec<AssetArchiveFileHeader>,
}

impl AssetArchiveMountPointHeader {
    pub fn new(version: u64, mount_point: String, assets: Vec<AssetArchiveFileHeader>) -> Self {
        Self {
            version,
            mount_point,
            assets,
        }
    }

    /// Get a reference to the asset archive mount point header's version.
    pub fn version(&self) -> &u64 {
        &self.version
    }

    /// Get a reference to the asset archive mount point header's mount point.
    pub fn mount_point(&self) -> &str {
        self.mount_point.as_str()
    }

    /// Get a reference to the asset archive mount point header's assets.
    pub fn assets(&self) -> &[AssetArchiveFileHeader] {
        self.assets.as_slice()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetArchiveFileHeader {
    asset_identifier: String,
    asset_format: String,
    offset: u64,
    compressed_size: u64,
    uncompressed_size: u64,
    compression_format: AssetArchiveCompressionFormat,
}

impl AssetArchiveFileHeader {
    pub fn new(
        asset_identifier: String,
        asset_format: String,
        offset: u64,
        compressed_size: u64,
        uncompressed_size: u64,
        compression_format: AssetArchiveCompressionFormat,
    ) -> Self {
        Self {
            asset_identifier,
            asset_format,
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

    /// Get a reference to the asset archive file header's asset identifier.
    pub fn asset_identifier(&self) -> &str {
        self.asset_identifier.as_str()
    }

    /// Get a reference to the asset archive file header's asset format.
    pub fn asset_format(&self) -> &str {
        self.asset_format.as_str()
    }
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AssetArchiveCompressionFormat {
    None = 0,
    LZ4 = 1,
}
