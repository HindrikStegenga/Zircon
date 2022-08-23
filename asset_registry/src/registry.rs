use crate::*;
use ahash::RandomState;
use dashmap::DashMap;
use uuid::Uuid;

pub type AssetSourceHandle = Uuid;

pub struct AssetRegistry {
    source: DashMap<AssetSourceHandle, Box<dyn AssetSource>>,
    assets: DashMap<u64, AssetDescriptor, ahash::RandomState>,
}

pub enum AssetRegistryError {
    AlreadyRegistered,
    UnknownAssetSource,
    UnknownAssetIdentifier,
}

impl Default for AssetRegistry {
    fn default() -> Self {
        Self {
            source: Default::default(),
            assets: Default::default(),
        }
    }
}

impl AssetRegistry {
    pub fn register_asset_source(
        &self,
        source: impl AssetSource,
    ) -> Result<AssetSourceHandle, AssetRegistryError> {
        let handle = source.get_handle();

        Err(AssetRegistryError::AlreadyRegistered)
    }

    pub fn unregister_asset_source(
        &self,
        source: AssetSourceHandle,
    ) -> Result<(), AssetRegistryError> {
        return Err(AssetRegistryError::UnknownAssetSource);
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
}
