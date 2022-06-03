use ::serde::{Deserialize, Serialize};
use uuid::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveId(Uuid);

impl ArchiveId {
    pub fn new(uuid: Uuid) -> Self {
        ArchiveId(uuid)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetId(u64);

impl AssetId {
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(xxhash_rust::xxh3::xxh3_64(id.as_ref().as_bytes()))
    }
}
