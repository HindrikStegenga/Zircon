use std::{fs::read_dir, path::Path};
mod error;
use serde::de::DeserializeOwned;

use crate::{
    archive::*,
    vfs::*,
    vfs::{archive_mount_point::ArchiveMountPoint, physical_mount_point::VfsPhysicalMountPoint},
    *,
};

use self::error::AssetSystemError;

pub struct AssetSystem {
    vfs: VirtualFileSystem,
}

impl Default for AssetSystem {
    fn default() -> Self {
        Self {
            vfs: Default::default(),
        }
    }
}

impl AssetSystem {
    pub fn load_asset_as<T: DeserializeOwned>(
        &self,
        mount_point: impl AsRef<str>,
        identifier: impl AsRef<str>,
    ) -> Result<T, AssetSystemError> {
        unimplemented!()
    }

    pub fn load_asset_as_blob_into(
        &self,
        mount_point: impl AsRef<str>,
        identifier: impl AsRef<str>,
        buffer: &mut Vec<u8>,
    ) -> Result<AssetDescriptor, AssetSystemError> {
        self.vfs
            .read_file_into(mount_point, identifier, buffer)
            .map_err(|e| e.into())
    }

    pub fn load_files_from_directory(
        &mut self,
        directory: impl AsRef<Path>,
        mount_point: impl AsRef<str>,
    ) -> Result<(), AssetSystemError> {
        let mnt = VfsPhysicalMountPoint::new(&mount_point, &directory)?;
        if !self.vfs.mount(mnt) {
            tagged_warn!(
                "Asset System",
                "Directory mount point was not mounted: {:#?} mount point: {:#?}",
                directory.as_ref(),
                mount_point.as_ref()
            );
        }
        tagged_success!(
            "Asset System",
            "Mounted files from directory: {:#?} mount point: {:#?}",
            directory.as_ref(),
            mount_point.as_ref()
        );
        Ok(())
    }

    pub fn load_archives_from_directory(
        &mut self,
        directory: impl AsRef<Path>,
        file_extension: impl AsRef<str>,
    ) -> Result<(), AssetSystemError> {
        let dir = read_dir(directory.as_ref())?;
        let valid_dir_entries = dir
            .filter_map(|d| match d {
                Err(e) => {
                    tagged_warn!("Asset System", "Invalid directory entry found: {}", e);
                    None
                }
                Ok(v) => Some(v),
            })
            .collect::<Vec<_>>();
        let valid_dir_entries = valid_dir_entries
            .iter()
            .filter_map(|d| match d.metadata() {
                Ok(m) => match m.is_file() {
                    true => match d.path().extension() {
                        Some(extension) => match extension.to_str() {
                            Some(s) => {
                                if s == file_extension.as_ref() {
                                    Some(d)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        },
                        None => None,
                    },
                    false => None,
                },
                Err(e) => {
                    tagged_warn!("Asset System", "Could not retrieve file metadata: {}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut counter = 0;
        for dir_entry in valid_dir_entries {
            let archive = AssetArchive::read_from_file(dir_entry.path())?;
            for mount_point in archive.header().mount_points() {
                let physical_mount =
                    ArchiveMountPoint::new(archive.path().into(), mount_point.clone());
                if !self.vfs.mount(physical_mount) {
                    tagged_warn!(
                        "Asset System",
                        "Archive mount point was not mounted: {:#?}",
                        archive.path()
                    );
                }
            }

            counter += 1;
        }
        tagged_success!(
            "Asset System",
            "Loaded {} asset archives from: {:#?}",
            counter,
            directory.as_ref()
        );

        Ok(())
    }
}
