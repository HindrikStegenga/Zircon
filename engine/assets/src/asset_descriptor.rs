use crate::formats::AssetSerializationFormat;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[macro_export]
macro_rules! asset_id {
    ($val:expr) => {
        $crate::AssetIdentifier::named(stringify!($val))
    };
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AssetIdentifier(u64);

impl From<u64> for AssetIdentifier {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<AssetIdentifier> for u64 {
    fn from(v: AssetIdentifier) -> Self {
        v.0
    }
}

impl Display for AssetIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

impl AssetIdentifier {
    pub const fn named(identifier: &str) -> Self {
        use xxhash_rust::const_xxh3::xxh3_64;
        Self(xxh3_64(str::as_bytes(identifier)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AssetSourceInfo {
    Archive(Uuid, usize),
    MappedFile,
    MappedDirectory,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetDescriptor {
    identifier: AssetIdentifier,
    version: u16,
    file_size: u32,
    format: AssetSerializationFormat,
    source_info: AssetSourceInfo,
}

impl AssetDescriptor {
    pub const fn new(
        identifier: AssetIdentifier,
        version: u16,
        byte_count: u32,
        format: AssetSerializationFormat,
        source_info: AssetSourceInfo,
    ) -> Self {
        Self {
            identifier,
            version,
            file_size: byte_count,
            format,
            source_info,
        }
    }

    pub const fn format(&self) -> AssetSerializationFormat {
        self.format
    }

    pub const fn byte_count(&self) -> u32 {
        self.file_size
    }

    pub const fn identifier(&self) -> AssetIdentifier {
        self.identifier
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub const fn source_info(&self) -> AssetSourceInfo {
        self.source_info
    }
}
