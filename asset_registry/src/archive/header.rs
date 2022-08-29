use crate::AssetIdentifier;
use ::serde::{Deserialize, Serialize};
use uuid::*;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Serialize, Deserialize, Clone, Hash)]
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
#[derive(Serialize, Deserialize, Clone, Copy, Hash)]
pub enum ArchiveCompressionFormat {
    None = 0,
    ZSTD = 1,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, Hash)]
pub enum AssetSerializationFormat {
    Binary = 0,
    Toml = 3,
    Unknown = 255,
}

impl From<&str> for AssetSerializationFormat {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "bin" => AssetSerializationFormat::Binary,
            "toml" => AssetSerializationFormat::Toml,
            _ => AssetSerializationFormat::Toml,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct FileHeader {
    #[serde(rename = "sid")]
    string_identifier: String,
    /// Hash of identifier. (Uses xxh3_64)
    #[serde(rename = "id")]
    id: AssetIdentifier,
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
    pub fn identifier(&self) -> &str {
        self.string_identifier.as_str()
    }

    pub fn id(&self) -> AssetIdentifier {
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
    pub fn new(
        identifier: String,
        format: AssetSerializationFormat,
        version: u16,
        offset: u64,
        byte_count: u64,
        compressed_byte_count: u64,
        compressed_hash: u64,
        compressed_format: ArchiveCompressionFormat,
    ) -> Self {
        let id = xxh3_64(&identifier.as_bytes()).into();
        Self {
            string_identifier: identifier,
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

    pub const MAX_FILE_HEADER_NAME_LEN: usize = 256;
}
