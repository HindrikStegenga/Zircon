use crate::{ArchiveId, MappedDirectoryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetSource {
    Archive(ArchiveId),
    MappedDirectory(MappedDirectoryId),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetDescriptor {
    version: u16,
    source: AssetSource,
}

impl AssetDescriptor {
    pub fn new(version: u16, source: AssetSource) -> Self {
        Self { version, source }
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn source(&self) -> &AssetSource {
        &self.source
    }
}
