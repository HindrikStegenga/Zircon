use crate::*;
use dashmap::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::fs::*;
use tokio::io::*;

use serde::{de::DeserializeOwned, Deserialize};
use utils::{dispatcher::Dispatcher, t_info, t_warn};

struct LoadedArchive {
    archive_header: ArchiveHeader,
    path: PathBuf,
}

struct MappedDirectory {
    path: PathBuf,
}

pub struct AssetRegistry {
    loaded_archives: DashMap<ArchiveId, LoadedArchive, ahash::RandomState>,
    mapped_directories: DashMap<MappedDirectoryId, MappedDirectory, ahash::RandomState>,
    assets: DashMap<u64, AssetDescriptor, ahash::RandomState>,
    dispatcher: Arc<Dispatcher>,
}

impl AssetRegistry {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        Self {
            loaded_archives: Default::default(),
            assets: Default::default(),
            mapped_directories: Default::default(),
            dispatcher,
        }
    }
}

async fn archive_from_file(path: &Path) -> std::result::Result<ArchiveHeader, AssetArchiveError> {
    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);
    if !AssetArchive::read_magic_value(&mut reader).await? {
        return Err(AssetArchiveError::InvalidMagicValue);
    };
    let header = AssetArchive::read_header(&mut reader).await?;
    Ok(header)
}

impl AssetRegistry {
    pub fn load_archive_at_path(self: &Arc<Self>, path: &impl AsRef<Path>) {
        let path: PathBuf = path.as_ref().into();
        let arc_self = Arc::clone(self);
        self.dispatcher.spawn_async(async move {
            let header = match archive_from_file(&path).await {
                Ok(v) => v,
                Err(e) => {
                    t_warn!("Could not read archive: {e}");
                    return;
                }
            };
            if arc_self.loaded_archives.contains_key(&header.uuid().into()) {
                t_warn!(
                    "Tried to load an archive that is already loaded: {}",
                    header.uuid()
                );
                return;
            }
            arc_self.store_assets_from_archive(&header);
            t_info!("Loaded archive into registry: {}", header.uuid());
        });
    }

    pub fn load_archive_at_path_blocking(self: &Arc<Self>, path: &impl AsRef<Path>) {
        let path: PathBuf = path.as_ref().into();
        let arc_self = Arc::clone(self);
        self.dispatcher.spawn_async_and_wait(async move {
            let header = match archive_from_file(&path).await {
                Ok(v) => v,
                Err(e) => {
                    t_warn!("Could not read archive: {e}");
                    return;
                }
            };
            if arc_self.loaded_archives.contains_key(&header.uuid().into()) {
                t_warn!(
                    "Tried to load an archive that is already loaded: {}",
                    header.uuid()
                );
                return;
            }
            arc_self.store_assets_from_archive(&header);
            t_info!("Loaded archive into registry: {}", header.uuid());
        });
    }

    fn store_assets_from_archive(self: &Arc<Self>, archive: &ArchiveHeader) {
        for file in archive.files() {
            let mut entry = match self.assets.get_mut(&file.id().into()) {
                Some(v) => v,
                None => {
                    self.assets.insert(
                        file.id().into(),
                        AssetDescriptor::new(
                            file.version(),
                            AssetSource::Archive(archive.uuid().into()),
                        ),
                    );
                    return;
                }
            };
            if file.version() > entry.value().version() {
                // Load the file.
                *entry = AssetDescriptor::new(
                    file.version(),
                    AssetSource::Archive(archive.uuid().into()),
                );
            } else if file.version() == entry.value().version() {
                t_warn!("Detected identically versioned assets!");
            }
        }
    }

    pub fn load_archives_in_directory(&self, path: &impl AsRef<Path>, extension: &impl AsRef<str>) {
    }

}
