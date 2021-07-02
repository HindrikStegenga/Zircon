use std::{error::Error, fs::read_dir, path::Path};

use magnetar_asset_library::{archive::*, vfs::*, *};

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
    pub fn load_archives_from_directory(
        &mut self,
        directory: impl AsRef<Path>,
        file_extension: impl AsRef<str>,
    ) -> Result<(), Box<dyn Error>> {
        let dir = read_dir(directory.as_ref())?;
        let valid_dir_entries = dir
            .filter_map(|d| match d {
                Err(e) => {
                    warn!("Invalid directory entry found: {}", e);
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
                    warn!("Could not retrieve file metadata: {}", e);
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut counter = 0;
        for dir_entry in valid_dir_entries {
            let archive = AssetArchive::read_from_file(dir_entry.path())?;
            counter += 1;
        }
        log!(
            "Loaded {} asset archives from: {:#?}",
            counter,
            directory.as_ref()
        );

        Ok(())
    }
}
