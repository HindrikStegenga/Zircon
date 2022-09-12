use crate::registry::*;
use crate::AssetIdentifier;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::Arc;
use utils::dispatcher::Dispatcher;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AssetError {
    InputOutputError,
    DeserializationFailure,
}

pub struct AssetCache {
    registry: AssetRegistry,
    dispatcher: Arc<Dispatcher>,
}

impl AssetCache {
    pub fn new(registry: AssetRegistry, dispatcher: Arc<Dispatcher>) -> Self {
        AssetCache {
            registry,
            dispatcher,
        }
    }

    pub fn load_typed_into_blocking<'a, 'b, T>(
        &'a self,
        identifier: AssetIdentifier,
        buffer: &'b mut [u8],
    ) -> Result<T, AssetError>
    where
        T: Deserialize<'b>,
    {
        let buf = self.load_blob_into_blocking(identifier, buffer)?;
        toml::from_slice(buf).map_err(|e| AssetError::DeserializationFailure)
    }

    pub fn load_blob_into_blocking<'a, 'b>(
        &'a self,
        identifier: AssetIdentifier,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], AssetError> {
        self.dispatcher
            .spawn_async_blocking(self.registry.load_asset_into(identifier, buffer))
            .map_err(|e| AssetError::InputOutputError)
    }
}
