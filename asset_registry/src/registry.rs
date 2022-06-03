use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc, RwLock},
};

use serde::{de::DeserializeOwned, Deserialize};
use uuid::Uuid;

use crate::*;

pub enum AssetLoadError {}

struct LoadedArchive {
    archive_header: ArchiveHeader,
    path: PathBuf,
}
struct AssetDescriptor {
    version: u16,
    archive: Uuid,
    is_loading: AtomicBool,
}

struct AssetRegistryState {
    loaded_archives: HashMap<Uuid, LoadedArchive, ahash::RandomState>,
    assets: HashMap<u64, AssetDescriptor, ahash::RandomState>,
}

pub struct AssetRegistry {
    state: Arc<AssetRegistryState>,
}

pub struct AssetLoader {
    registry: Arc<RwLock<AssetRegistryState>>,
}

impl AssetLoader {
    pub fn load_archive_from_path(&mut self, path: &impl AsRef<Path>) {}

    pub fn unload_archive(&mut self, archive_id: ArchiveId) {}

    pub fn load_blob(
        &self,
        id: AssetId,
        on_complete: impl FnOnce(Result<Vec<u8>, AssetLoadError>) + Send,
    ) {
    }

    pub fn load_blob_async(&self, id: AssetId) {}

    pub fn load_as_type<T: DeserializeOwned>(&self, id: AssetId) {}

    pub fn load_as_type_async<T: DeserializeOwned>(&self, id: AssetId) {}

    pub fn load_as_type_into_buffer(&self) {}

    pub fn load_as_type_into_buffer_async(&self) {}
}
