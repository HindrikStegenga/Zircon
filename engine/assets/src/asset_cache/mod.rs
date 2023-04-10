mod asset_buffer;
#[cfg(test)]
mod tests;

use crate::{AssetIdentifier, AssetRegistry};
use asset_buffer::*;
use crossbeam::queue::ArrayQueue;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Weak;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use utils::dispatcher::Dispatcher;
use utils::t_warn;

pub struct AssetHandle<T> {
    _phantom: PhantomData<T>,
    reference: Arc<AssetBuffer>,
}
// impl AssetHandle<[u8]> {
//     fn read(&self) -> Option<&[u8]> {
//         self.reference.try_read()
//     }
// }
impl<T: Sized> AssetHandle<T> {
    fn read(&self) -> Option<&T> {
        let slice = self.reference.try_read()?;
        unsafe {
            // TODO: Deserialization.
            return Some(&*(slice.as_ptr() as *const T));
        }
    }
}
impl<T> AssetHandle<T> {
    pub fn state(&self) -> AssetState {
        self.reference.state()
    }
}
impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        Self {
            _phantom: Default::default(),
            reference: Arc::clone(&self.reference),
        }
    }
}

#[derive(Debug)]
pub enum AssetCacheError {
    UnknownAsset,
    //TODO: Remove this one in the future?
    NoBufferAvailable,
    DeserializationFailure,
}

pub struct AssetCache<R: AsyncReadExt + AsyncSeekExt + Unpin + Send = File> {
    dispatcher: Arc<Dispatcher>,
    loaded_raw_buffers: DashMap<AssetIdentifier, Weak<AssetBuffer>>,
    registry: Arc<AssetRegistry<R>>,
    // TODO: Smarter buffer sizing and the likes
    buffers: Arc<ArrayQueue<Vec<u8>>>,
}

impl<R: AsyncReadExt + AsyncSeekExt + Unpin + Send + 'static> AssetCache<R> {
    pub fn new(registry: Arc<AssetRegistry<R>>, dispatcher: Arc<Dispatcher>) -> Self {
        const BUF_COUNT: usize = 1024;
        let buffers = ArrayQueue::new(BUF_COUNT);
        for _ in 0..BUF_COUNT {
            buffers
                .push(Vec::with_capacity(4096))
                .expect("Unable to push elements during construction.");
        }
        Self {
            dispatcher,
            loaded_raw_buffers: DashMap::default(),
            registry,
            buffers: Arc::new(buffers),
        }
    }

    pub fn request_binary(
        &self,
        asset_id: AssetIdentifier,
    ) -> Result<AssetHandle<&[u8]>, AssetCacheError> {
        if let Some(value) = self.loaded_raw_buffers.get(&asset_id) {
            if let Some(item) = value.value().upgrade() {
                return Ok(AssetHandle {
                    _phantom: Default::default(),
                    reference: item,
                });
            }
            self.loaded_raw_buffers.remove(&asset_id);
        }

        let Ok(descriptor) = self.registry.get_asset_descriptor(asset_id) else {
            return Err(AssetCacheError::UnknownAsset);
        };
        let Some(mut buffer) = self.buffers.pop() else {
            return Err(AssetCacheError::NoBufferAvailable);
        };
        buffer.clear();
        buffer.resize(descriptor.byte_count() as usize, 0);
        let registry = Arc::clone(&self.registry);
        let asset_buffer = AssetBuffer::new(asset_id, Arc::clone(&self.buffers));
        let return_value = AssetHandle::<&[u8]> {
            _phantom: Default::default(),
            reference: Arc::clone(&asset_buffer),
        };
        let buffers = Arc::clone(&self.buffers);
        self.loaded_raw_buffers
            .insert(asset_id, Arc::downgrade(&return_value.reference));
        self.dispatcher.spawn_async(async move {
            match registry.load_asset_into(asset_id, &mut buffer).await {
                Ok(slice) => {
                    let len = slice.len();
                    asset_buffer.set_available(buffer, len);
                    // TODO: Notification mechanism?
                }
                Err(e) => {
                    t_warn!("Asset loading error: {:#?}", e);
                    asset_buffer.set_failed();
                    buffer.clear();
                    match buffers.push(buffer) {
                        Ok(_) => {}
                        Err(buf) => {
                            t_warn!(
                                "Could not recycle buffer. Buffer is lost: {}",
                                buf.capacity()
                            );
                        }
                    }
                }
            };
        });
        Ok(return_value)
    }

    pub fn request<T: DeserializeOwned>(
        &self,
        asset_id: AssetIdentifier,
    ) -> Result<AssetHandle<T>, AssetCacheError> {
        todo!()
    }
}
