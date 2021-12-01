use std::path::PathBuf;

use crate::{
    archive::{AssetArchive, AssetArchiveMountPointHeader},
    AssetDescriptor,
};

use super::{error::VfsError, VfsMountPoint};

pub struct ArchiveMountPoint {
    path: PathBuf,
    header: AssetArchiveMountPointHeader,
}

impl ArchiveMountPoint {
    pub fn new(path: PathBuf, header: AssetArchiveMountPointHeader) -> Self {
        Self { path, header }
    }

    pub fn from_archive(archive: &AssetArchive) -> Vec<ArchiveMountPoint> {
        archive
            .header()
            .mount_points()
            .iter()
            .map(|a| ArchiveMountPoint::new(archive.path().into(), a.clone()))
            .collect()
    }
}

impl VfsMountPoint for ArchiveMountPoint {
    fn identifier(&self) -> &str {
        self.header.mount_point()
    }

    fn has_file(&self, identifier: &str) -> bool {
        self.header
            .assets()
            .iter()
            .find(|a| a.asset_identifier() == identifier)
            .is_some()
    }

    fn get_asset_descriptor(&self, identifier: &str) -> Option<crate::AssetDescriptor> {
        let asset_header = self
            .header
            .assets()
            .iter()
            .find(|e| e.asset_identifier() == identifier)?;
        Some(AssetDescriptor::new(
            self.header.mount_point().into(),
            asset_header.asset_identifier().into(),
            asset_header.asset_format().into(),
        ))
    }

    fn version(&self) -> u64 {
        *self.header.version()
    }

    fn load_asset_into(
        &self,
        identifier: &str,
        buffer: &mut Vec<u8>,
    ) -> Result<AssetDescriptor, VfsError> {
        let asset_header = self
            .header
            .assets()
            .iter()
            .find(|e| e.asset_identifier() == identifier)
            .ok_or("Asset not found")
            .map_err(|_| VfsError::FileNotFound)?;
        let result = AssetArchive::read_file_into(&self.path, asset_header, buffer);
        match result {
            Ok(_) => Ok(AssetDescriptor::new(
                self.header.mount_point().to_string(),
                identifier.to_string(),
                asset_header.asset_format().to_string(),
            )),
            Err(e) => Err(VfsError::Other(Box::from(e))),
        }
    }
}
