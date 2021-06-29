use std::{str::Utf8Error, usize};

use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetArchive {
    asset_descriptors: Vec<AssetDescriptor>,
    blobs: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDescriptor {
    id: String,
    format: AssetFormat,
    path: Option<String>,
    blob_properties: AssetBlobProperties,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AssetBlobProperties {
    format: AssetBlobFormat,
    size: usize,
    offset: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetBlobFormat {
    Blob,
    LZ4Blob,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetFormat {
    format: [u8; 3],
}

impl AssetFormat {
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.format)
    }
}
