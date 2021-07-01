use crate::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::{fs::read_dir, path::*};

use crate::{vfs::*, AssetDescriptor};

const DEFAULT_INDEX_FILE_NAME: &'static str = "index.yaml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetIndex {
    version: usize,
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
    pub fn new(mount_point: &impl AsRef<str>, directory: &impl AsRef<Path>) -> Result<Self> {
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
                            warn!("{:#?}", err)
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
}

impl VfsMountPoint for VfsPhysicalMountPoint {
    fn identifier(&self) -> &str {
        &self.mount_point
    }

    fn has_file(&self, identifier: &str) -> bool {
        match &self.index {
            Some(index) => {
                // Uses the index file to load files. All files are by definition uncompressed.
                index.has_file(identifier)
            }
            None => {
                // File structure is used instead.
                let directory = match read_dir(&self.directory) {
                    Ok(d) => d,
                    Err(e) => failure!("{:#?}", e),
                };
                for file in directory {
                    match file {
                        Ok(d) => {
                            if d.file_name() == identifier {
                                return true;
                            }
                        }
                        Err(e) => warn!("Could not open file: {:#?}", e),
                    }
                }

                false
            }
        }
    }

    fn version(&self) -> usize {
        if let Some(index) = &self.index {
            index.version
        } else {
            0
        }
    }
}
