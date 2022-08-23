use crate::{ArchiveId, MappedDirectoryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetSource {
    Archive(ArchiveId),
    MappedDirectory(MappedDirectoryId),
}


