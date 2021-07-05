use crate::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, DirEntry, File},
    io::{BufReader, Read, Seek, SeekFrom},
    path::*,
};

use crate::{vfs::*, AssetDescriptor};

const DEFAULT_INDEX_FILE_NAME: &'static str = "index.yaml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetIndex {
    version: Option<u64>,
    files: Vec<AssetDescriptor>,
}

impl AssetIndex {
    pub fn has_file(&self, identifier: &str) -> bool {
        self.files
            .iter()
            .find(|d| d.identifier() == identifier)
            .is_some()
    }
}

pub struct VfsPhysicalMountPoint {
    mount_point: String,
    directory: PathBuf,
    index: Option<AssetIndex>,
}

impl VfsPhysicalMountPoint {
    pub fn new(
        mount_point: &impl AsRef<str>,
        directory: &impl AsRef<Path>,
    ) -> Result<Self, std::io::Error> {
        let mut mount = Self {
            mount_point: mount_point.as_ref().into(),
            directory: directory.as_ref().into(),
            index: None,
        };
        let directory = read_dir(directory)?;
        for f in directory {
            if let Ok(f) = f {
                if f.file_name() == DEFAULT_INDEX_FILE_NAME {
                    let bytes = load_file_bin(f.path())?;
                    match serde_yaml::from_slice::<AssetIndex>(&bytes) {
                        Ok(index) => {
                            mount.index = index.into();
                            break;
                        }
                        Err(err) => {
                            tagged_warn!("VFS", "{:#?}", err)
                        }
                    }
                }
            }
        }

        Ok(mount)
    }

    pub fn asset_index(&self) -> &Option<AssetIndex> {
        &self.index
    }

    fn find_dir_entry(&self, identifier: &str) -> Result<DirEntry, VfsError> {
        read_dir(&self.directory)?
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| match f.file_type() {
                Ok(f) => f.is_file(),
                Err(_) => false,
            })
            .find(|e| {
                if let Some(p) = e.path().file_stem() {
                    p == identifier
                } else {
                    false
                }
            })
            .ok_or(VfsError::FileNotFound)
    }
}

impl VfsMountPoint for VfsPhysicalMountPoint {
    fn identifier(&self) -> &str {
        &self.mount_point
    }

    fn has_file(&self, identifier: &str) -> bool {
        return match &self.index {
            Some(index) => {
                // Uses the index file to load files. All files are by definition uncompressed.
                if index.has_file(identifier) {
                    return true;
                } else {
                    // Not in index.yaml, load from direcotry instead.
                    self.find_dir_entry(identifier).is_ok()
                }
            }
            None => {
                // File structure is used instead.
                self.find_dir_entry(identifier).is_ok()
            }
        };
    }

    fn version(&self) -> u64 {
        if let Some(index) = &self.index {
            return index.version.unwrap_or_default();
        } else {
            0
        }
    }

    fn get_asset_descriptor(&self, identifier: &str) -> Option<AssetDescriptor> {
        let find = || -> Option<AssetDescriptor> {
            let fd = self.find_dir_entry(identifier).ok()?;
            let path = fd.path();
            let extension = match path.extension() {
                Some(ext) => ext.to_str(),
                None => None,
            };
            AssetDescriptor::new(
                self.mount_point.clone(),
                identifier.to_string(),
                extension.unwrap_or("").to_string(),
            )
            .into()
        };

        if let Some(asset_index) = self.asset_index() {
            if let Some(file) = asset_index
                .files
                .iter()
                .find(|e| e.identifier() == identifier)
            {
                file.clone().into()
            } else {
                find()
            }
        } else {
            find()
        }
    }

    fn load_asset_into(
        &self,
        identifier: &str,
        buffer: &mut Vec<u8>,
    ) -> Result<AssetDescriptor, VfsError> {
        let fd = self.find_dir_entry(identifier)?;
        let file = File::open(fd.path())?;
        let mut buf_reader = BufReader::new(file);
        buf_reader.seek(SeekFrom::Start(0))?;
        buf_reader.read_to_end(buffer)?;
        let path = fd.path();
        let extension = match path.extension() {
            Some(ext) => ext.to_str(),
            None => None,
        };
        Ok(AssetDescriptor::new(
            self.mount_point.clone(),
            identifier.to_string(),
            extension.unwrap_or("").to_string(),
        ))
    }
}
