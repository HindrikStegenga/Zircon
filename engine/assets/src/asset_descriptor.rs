use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    source_info: AssetSourceInfo,
}

impl AssetDescriptor {
    pub const fn new(
        identifier: AssetIdentifier,
        version: u16,
        source_info: AssetSourceInfo,
    ) -> Self {
        Self {
            identifier,
            version,
            source_info,
        }
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
