use crate::registry::*;
use crate::{AssetBuffer, AssetDescriptor, AssetIdentifier, AssetSerializationFormat};
use crossbeam::queue::ArrayQueue;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use utils::dispatcher::Dispatcher;
use utils::{t_info, t_warn};

const INITIAL_BUFFER_COUNT: usize = 64;
const INITIAL_BUFFER_SIZE: usize = 8192;

const BUFFER_QUEUE_SIZE: usize = 256;
const ASSET_QUEUE_SIZE: usize = 256;
const ERROR_QUEUE_SIZE: usize = 1024;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AssetError {
    InputOutputError,
    DeserializationFailure,
    UnknownAssetIdentifier,
    NoBufferAvailable,
}

impl Display for AssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::InputOutputError => f.write_str("InputOutputError"),
            AssetError::DeserializationFailure => f.write_str("DeserializationFailure"),
            AssetError::UnknownAssetIdentifier => f.write_str("UnknownAssetIdentifier"),
            AssetError::NoBufferAvailable => f.write_str("NoBufferAvailable"),
        }
    }
}

#[derive(Debug)]
pub struct AvailableAsset {
    descriptor: AssetDescriptor,
    buffer: AssetBuffer,
}

impl AvailableAsset {
    pub fn buffer(&self) -> &[u8] {
        self.buffer.as_ref()
    }
    pub fn descriptor(&self) -> &AssetDescriptor {
        &self.descriptor
    }
}

#[derive(Debug)]
pub struct UnavailableAsset {
    asset_id: AssetIdentifier,
    reason: AssetError,
}

pub struct AssetCache {
    registry: Arc<AssetRegistry>,
    loading_state: DashMap<AssetIdentifier, bool>,
    dispatcher: Arc<Dispatcher>,
    available_buffers: Arc<ArrayQueue<Vec<u8>>>,
    available_assets: Arc<ArrayQueue<AvailableAsset>>,
    unavailable_assets: Arc<ArrayQueue<UnavailableAsset>>,
}

impl AssetCache {
    pub fn print_available_assets(&self) {
        self.registry.print_available_assets();
    }

    fn request_descriptor_and_buffer(
        &self,
        identifier: AssetIdentifier,
    ) -> Result<(AssetDescriptor, Vec<u8>), AssetError> {
        let descriptor = match self.registry.get_asset_descriptor(identifier) {
            Ok(v) => v,
            Err(_) => return Err(AssetError::UnknownAssetIdentifier),
        };
        let buffer = match self.available_buffers.pop() {
            Some(v) => v,
            None => return Err(AssetError::NoBufferAvailable),
        };
        Ok((descriptor, buffer))
    }

    fn deserialize_asset_from_buffer<T>(
        descriptor: &AssetDescriptor,
        buffer: &[u8],
    ) -> Result<T, AssetError>
    where
        T: DeserializeOwned,
    {
        match descriptor.format() {
            AssetSerializationFormat::Binary => Err(AssetError::DeserializationFailure),
            AssetSerializationFormat::Toml => {
                toml::from_slice(buffer).map_err(|_| AssetError::DeserializationFailure)
            }
            AssetSerializationFormat::Unknown => {
                todo!()
            }
        }
    }
}

impl AssetCache {
    pub fn new(registry: AssetRegistry, dispatcher: Arc<Dispatcher>) -> Self {
        let cache = AssetCache {
            registry: Arc::new(registry),
            loading_state: Default::default(),
            dispatcher,
            available_buffers: Arc::new(ArrayQueue::new(BUFFER_QUEUE_SIZE)),
            available_assets: Arc::new(ArrayQueue::new(ASSET_QUEUE_SIZE)),
            unavailable_assets: Arc::new(ArrayQueue::new(ERROR_QUEUE_SIZE)),
        };
        (0..INITIAL_BUFFER_COUNT).into_iter().for_each(|_| {
            cache
                .available_buffers
                .push(Vec::with_capacity(INITIAL_BUFFER_SIZE))
                .expect("{unreachable}")
        });

        cache
    }

    pub fn request_asset(&self, identifier: AssetIdentifier) {
        let (descriptor, buffer) = match self.request_descriptor_and_buffer(identifier) {
            Ok(v) => v,
            Err(e) => {
                t_warn!("Could not get descriptor/buffer: {e}");
                self.unavailable_assets
                    .push(UnavailableAsset {
                        asset_id: identifier,
                        reason: e,
                    })
                    .expect("Could not make asset unavailable.");
                return;
            }
        };
        let registry = Arc::clone(&self.registry);
        let available_bufs = Arc::clone(&self.available_buffers);
        let available_assets = Arc::clone(&self.available_assets);
        let unavailable_assets = Arc::clone(&self.unavailable_assets);
        self.dispatcher.spawn_async(async move {
            let mut buffer = buffer;
            buffer.resize(descriptor.byte_count() as usize, 0);
            let used_size = match registry.load_asset_into(identifier, &mut buffer).await {
                Ok(result) => result.len(),
                Err(e) => {
                    t_warn!("Unable to load: {:#?} | {:#?}", identifier, e);
                    let _ = unavailable_assets
                        .push(UnavailableAsset {
                            asset_id: identifier,
                            reason: AssetError::InputOutputError,
                        })
                        .map_err(|e| t_warn!("Unable to make unavailable: {:#?}", e.asset_id));
                    return;
                }
            };
            match available_assets.push(AvailableAsset {
                descriptor,
                buffer: AssetBuffer::new(buffer, used_size, Arc::clone(&available_bufs)),
            }) {
                Ok(_) => {
                    t_info!("Loaded asset: {identifier}");
                }
                Err(e) => {
                    t_warn!("Unable to load: {:#?} | {:#?}", identifier, e);
                    let buf = vec![];
                    //std::mem::swap(&mut e.buffer, &mut buf);
                    let _ = available_bufs.push(buf).map_err(|e| {
                        let _ = unavailable_assets.push(UnavailableAsset {
                            asset_id: identifier,
                            reason: AssetError::InputOutputError,
                        });
                        let _ = available_bufs.push(e).map_err(|_| {
                            t_warn!("Could not recycle buffer!");
                        });
                    });
                    let _ = unavailable_assets
                        .push(UnavailableAsset {
                            asset_id: identifier,
                            reason: AssetError::InputOutputError,
                        })
                        .map_err(|_| {
                            t_warn!("Could not make asset unavailable.");
                            ()
                        });
                }
            }
        });
    }

    pub fn load_blob_into<'a, 'b>(
        &'a self,
        identifier: AssetIdentifier,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], AssetError> {
        self.dispatcher
            .spawn_async_blocking(self.registry.load_asset_into(identifier, buffer))
            .map_err(|_| AssetError::InputOutputError)
    }

    pub fn load_typed_into<T>(
        &self,
        identifier: AssetIdentifier,
        buffer: &mut [u8],
    ) -> Result<T, AssetError>
    where
        T: DeserializeOwned,
    {
        let descriptor = match self.registry.get_asset_descriptor(identifier) {
            Ok(d) => d,
            Err(_) => return Err(AssetError::UnknownAssetIdentifier),
        };
        let buf = self.load_blob_into(identifier, buffer)?;
        Self::deserialize_asset_from_buffer(&descriptor, buf)
    }

    pub fn load_blob(
        &self,
        identifier: AssetIdentifier,
    ) -> Result<(AssetDescriptor, AssetBuffer), AssetError> {
        let (descriptor, mut buffer) = self.request_descriptor_and_buffer(identifier)?;
        buffer.resize(descriptor.byte_count() as usize, 0);
        let byte_count = self
            .dispatcher
            .spawn_async_blocking(self.registry.load_asset_into(identifier, &mut buffer))
            .map_err(|_| AssetError::InputOutputError)
            .map(|e| e.len())?;
        Ok((
            descriptor,
            AssetBuffer::new(buffer, byte_count, Arc::clone(&self.available_buffers)),
        ))
    }

    pub fn load_typed<T>(&self, identifier: AssetIdentifier) -> Result<T, AssetError>
    where
        T: DeserializeOwned,
    {
        let (descriptor, buffer) = self.load_blob(identifier)?;
        Self::deserialize_asset_from_buffer(&descriptor, buffer.as_ref())
    }
}
