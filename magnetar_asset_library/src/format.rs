use std::{str::Utf8Error, usize};

use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetArchive {
    asset_descriptors: Vec<AssetDescriptor>,
    blobs: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDescriptor {
    identifier: String,
    format: AssetFormat,
    path: Option<String>,
    #[serde(default)]
    blob_properties: AssetBlobProperties,
}

impl AssetDescriptor {
    /// Get a reference to the asset descriptor's identifier.
    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    /// Get a reference to the asset descriptor's format.
    pub fn format(&self) -> &AssetFormat {
        &self.format
    }

    /// Get a reference to the asset descriptor's path.
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Get a reference to the asset descriptor's blob properties.
    pub fn blob_properties(&self) -> &AssetBlobProperties {
        &self.blob_properties
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AssetBlobProperties {
    format: AssetBlobFormat,
    // size of the blob, if 0, size is whole file.
    size: usize,
    offset: usize,
}

impl Default for AssetBlobProperties {
    fn default() -> Self {
        Self {
            format: AssetBlobFormat::Uncompressed,
            size: 0,
            offset: 0,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetBlobFormat {
    Uncompressed = 0,
    LZ4 = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetFormat(String);

impl AssetFormat {
    // pub fn as_str(&self) -> Result<&str, Utf8Error> {
    //     std::str::from_utf8(&self.0)
    // }
}
