use hashbrown::*;
use std::sync::Arc;

struct LoadedArchive {}
struct AssetDescriptor {}

pub struct AssetRegistry {
    loaded_archives: Vec<LoadedArchive>,
    assets: HashMap<u64, AssetDescriptor>,
}

pub struct AssetLoader {
    registry: Arc<AssetRegistry>,
}
