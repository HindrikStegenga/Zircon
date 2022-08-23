use ::serde::{Deserialize, Serialize};
use uuid::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ArchiveId(Uuid);

impl ArchiveId {
    pub fn new(uuid: Uuid) -> Self {
        ArchiveId(uuid)
    }
}

impl From<Uuid> for ArchiveId {
    fn from(v: Uuid) -> Self {
        Self::new(v)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AssetId(u64);

impl AssetId {
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(xxhash_rust::xxh3::xxh3_64(id.as_ref().as_bytes()))
    }
}

impl From<u64> for AssetId {
    fn from(id: u64) -> Self {
        AssetId(id)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MappedDirectoryId(Uuid);

impl MappedDirectoryId {
    pub fn new(uuid: Uuid) -> Self {
        MappedDirectoryId(uuid)
    }
}

impl From<Uuid> for MappedDirectoryId {
    fn from(v: Uuid) -> Self {
        Self::new(v)
    }
}
