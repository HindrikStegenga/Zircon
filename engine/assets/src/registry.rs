use crate::*;
use ahash::RandomState;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::io;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetSourceHandle {
    AssetArchive(Uuid),
    MappedDirectory(u64),
    MappedFile(u64),
}

struct MappedFile {}
struct MappedDirectory {}

pub struct AssetRegistry {
    registered_archives: DashMap<Uuid, AssetArchive>,
    registered_files: DashMap<u64, MappedFile>,
    registered_directory_mappings: DashMap<u64, MappedDirectory>,
    assets: DashMap<AssetIdentifier, AssetDescriptor, RandomState>,
}
#[derive(Debug)]
pub enum AssetRegistryError {
    BufferTooSmall,
    AlreadyRegistered,
    UnknownAssetSource,
    UnknownAssetIdentifier,
    InvalidFile,
    DecompressionFailure,
    InputOutput(io::Error),
}

impl Default for AssetRegistry {
    fn default() -> Self {
        Self {
            registered_archives: Default::default(),
            registered_files: Default::default(),
            registered_directory_mappings: Default::default(),
            assets: Default::default(),
        }
    }
}

impl From<AssetArchiveError> for AssetRegistryError {
    fn from(error: AssetArchiveError) -> Self {
        match error {
            AssetArchiveError::InvalidMagicValue => Self::InvalidFile,
            AssetArchiveError::InvalidHeaderHash => Self::InvalidFile,
            AssetArchiveError::UnknownAssetIdentifier => Self::UnknownAssetIdentifier,
            AssetArchiveError::HeaderDeserializationError(_) => Self::DecompressionFailure,
            AssetArchiveError::InputOutput(e) => Self::InputOutput(e),
            AssetArchiveError::BufferTooSmall => Self::BufferTooSmall,
        }
    }
}

impl AssetRegistry {
    pub fn register_asset_archive(
        &self,
        asset_archive: AssetArchive,
    ) -> Result<AssetSourceHandle, (AssetRegistryError, AssetArchive)> {
        let handle: Uuid = asset_archive.header().uuid();
        return match self.registered_archives.entry(handle) {
            Entry::Vacant(vacant) => {
                let archive_ref = vacant.insert(asset_archive);
                let inserted_archive = archive_ref.value();
                inserted_archive
                    .header()
                    .files()
                    .iter()
                    .enumerate()
                    .for_each(|(file_offset, file_header)| {
                        match self.assets.entry(file_header.id()) {
                            Entry::Occupied(mut entry) => {
                                // If existing version is larger or equal, don't insert it.
                                if entry.get().version() >= file_header.version() {
                                    return;
                                }
                                entry.insert(AssetDescriptor::new(
                                    file_header.id(),
                                    file_header.version(),
                                    file_header.byte_count(),
                                    file_header.format(),
                                    AssetSourceInfo::Archive(handle, file_offset),
                                ));
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(AssetDescriptor::new(
                                    file_header.id(),
                                    file_header.version(),
                                    file_header.byte_count(),
                                    file_header.format(),
                                    AssetSourceInfo::Archive(handle, file_offset),
                                ));
                            }
                        }
                    });
                Ok(AssetSourceHandle::AssetArchive(handle))
            }
            Entry::Occupied(_) => Err((AssetRegistryError::AlreadyRegistered, asset_archive)),
        };
    }

    pub fn register_mapped_file(&self) {
        todo!()
    }

    pub fn register_mapped_directory(&self) {
        todo!()
    }

    pub fn contains_asset(&self, identifier: AssetIdentifier) -> bool {
        self.assets.contains_key(&identifier)
    }

    pub fn get_asset_descriptor(
        &self,
        identifier: AssetIdentifier,
    ) -> Result<AssetDescriptor, AssetRegistryError> {
        return if let Some(descriptor) = self.assets.get(&identifier) {
            Ok(*descriptor.value())
        } else {
            Err(AssetRegistryError::UnknownAssetIdentifier)
        };
    }

    pub async fn load_asset_into<'a, 'b>(
        &'a self,
        identifier: AssetIdentifier,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], AssetRegistryError> {
        return match self.assets.get(&identifier) {
            None => Err(AssetRegistryError::UnknownAssetIdentifier),
            Some(descriptor_ref) => {
                return match descriptor_ref.source_info() {
                    AssetSourceInfo::Archive(handle, offset) => {
                        return match self.registered_archives.get(&handle) {
                            None => Err(AssetRegistryError::UnknownAssetSource),
                            Some(archive_ref) => {
                                match archive_ref.value().read_asset_into(offset, buffer).await {
                                    Ok(buf) => Ok(buf),
                                    Err(e) => Err(e.into()),
                                }
                            }
                        };
                    }
                    AssetSourceInfo::MappedDirectory => {
                        todo!()
                    }
                    AssetSourceInfo::MappedFile => {
                        todo!()
                    }
                }
            }
        };
    }
}
