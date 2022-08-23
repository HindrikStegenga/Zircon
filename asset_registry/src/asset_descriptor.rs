use crate::registry::AssetSourceHandle;
use serde::{Deserialize, Serialize};

pub type AssetIdentifier = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetDescriptor {
    identifier: AssetIdentifier,
    version: u16,
    source: AssetSourceHandle,
}

impl AssetDescriptor {
    pub const fn new(identifier: AssetIdentifier, version: u16, source: AssetSourceHandle) -> Self {
        Self {
            identifier,
            version,
            source,
        }
    }

    pub const fn identifier(&self) -> AssetIdentifier {
        self.identifier
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub const fn source(&self) -> &AssetSourceHandle {
        &self.source
    }
}
