use std::usize;

use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDescriptor {
    mount: String,
    identifier: String,
    format: String,
}

impl AssetDescriptor {
    pub fn new(mount: String, identifier: String, format: String) -> Self {
        Self {
            mount: mount.to_lowercase(),
            identifier: identifier.to_lowercase(),
            format: format.to_lowercase(),
        }
    }

    /// Get a reference to the asset descriptor's identifier.
    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    /// Get a reference to the asset descriptor's mount.
    pub fn mount(&self) -> &str {
        self.mount.as_str()
    }

    /// Get a reference to the asset descriptor's format.
    pub fn format(&self) -> &str {
        self.format.as_str()
    }
}
