use crate::formats::*;
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
    offset: u32,
    #[serde(rename = "bc")]
    byte_count: u32,
    #[serde(rename = "cbc")]
    compressed_byte_count: u32,
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

    pub fn format(&self) -> AssetSerializationFormat {
        self.format
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn byte_count(&self) -> u32 {
        self.byte_count
    }

    pub fn compressed_byte_count(&self) -> u32 {
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
        offset: u32,
        byte_count: u32,
        compressed_byte_count: u32,
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
