use ::serde::{Deserialize, Serialize};
use arrayvec::ArrayString;
use uuid::*;

#[derive(Serialize, Deserialize, Hash)]
pub struct ArchiveHeader {
    #[serde(rename = "uid")]
    uuid: Uuid,
    #[serde(rename = "fls")]
    files: Vec<FileHeader>,
}

impl ArchiveHeader {
    pub const fn new(uuid: Uuid, files: Vec<FileHeader>) -> Self {
        Self { uuid, files }
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn files(&self) -> &[FileHeader] {
        self.files.as_ref()
    }
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Hash)]
pub enum ArchiveCompressionFormat {
    None = 0,
    ZSTD = 1,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Hash)]
pub enum AssetSerializationFormat {
    None = 0,
    JSON = 1,
    YAML = 2,
    TOML = 3,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct FileHeader {
    #[serde(rename = "sid")]
    identifier: ArrayString<{ FileHeader::FILE_HEADER_NAME_LEN }>,
    #[serde(rename = "id")]
    id: u64,
    #[serde(rename = "f")]
    format: AssetSerializationFormat,
    #[serde(rename = "v")]
    version: u16,
    #[serde(rename = "o")]
    offset: u64,
    #[serde(rename = "bc")]
    byte_count: u64,
    #[serde(rename = "cbc")]
    compressed_byte_count: u64,
    /// Hash of the compressed file. (Uses xxh3_64)
    #[serde(rename = "ch")]
    compressed_hash: u64,
    #[serde(rename = "cf")]
    compressed_format: ArchiveCompressionFormat,
}

impl FileHeader {
    pub fn identifier(&self) -> ArrayString<{ FileHeader::FILE_HEADER_NAME_LEN }> {
        self.identifier
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn format(&self) -> &AssetSerializationFormat {
        &self.format
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn byte_count(&self) -> u64 {
        self.byte_count
    }

    pub fn compressed_byte_count(&self) -> u64 {
        self.compressed_byte_count
    }

    pub fn compressed_hash(&self) -> u64 {
        self.compressed_hash
    }

    pub fn compressed_format(&self) -> &ArchiveCompressionFormat {
        &self.compressed_format
    }
}

impl FileHeader {
    pub const fn new(
        identifier: ArrayString<{ FileHeader::FILE_HEADER_NAME_LEN }>,
        id: u64,
        format: AssetSerializationFormat,
        version: u16,
        offset: u64,
        byte_count: u64,
        compressed_byte_count: u64,
        compressed_hash: u64,
        compressed_format: ArchiveCompressionFormat,
    ) -> Self {
        Self {
            identifier,
            id,
            format,
            version,
            offset,
            byte_count,
            compressed_byte_count,
            compressed_hash,
            compressed_format,
        }
    }

    pub const FILE_HEADER_NAME_LEN: usize = 128;
}
