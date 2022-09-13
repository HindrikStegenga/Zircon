use crate::registry::*;
use crate::{AssetDescriptor, AssetIdentifier, AssetSerializationFormat};
use async_channel::{SendError, TryRecvError, TrySendError};
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::future::Future;
use std::sync::Arc;
use utils::dispatcher::Dispatcher;
use utils::t_warn;

const INITIAL_BUFFER_COUNT: usize = 32;
const INITIAL_BUFFER_SIZE: usize = 8192;
const FILE_QUEUE_SIZE: usize = 256;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AssetError {
    InputOutputError,
    DeserializationFailure,
    UnknownAssetIdentifier,
}

#[derive(Debug)]
pub struct LoadedFileState {
    buffer: Vec<u8>,
    len: usize,
    buf_sender: async_channel::Sender<Vec<u8>>,
}

impl Drop for LoadedFileState {
    fn drop(&mut self) {
        let mut buffer = vec![];
        std::mem::swap(&mut self.buffer, &mut buffer);
        match self.buf_sender.try_send(buffer) {
            Ok(_) => return,
            Err(e) => match e {
                TrySendError::Full(buf) => match self.buf_sender.send_blocking(buf) {
                    Ok(_) => return,
                    Err(e) => {
                        t_warn!("{}", e);
                    }
                },
                TrySendError::Closed(_) => (),
            },
        }
    }
}

pub struct AssetCache {
    registry: Arc<AssetRegistry>,
    loading_state: DashMap<AssetIdentifier, bool>,
    dispatcher: Arc<Dispatcher>,
    buffer_queue_sender: async_channel::Sender<Vec<u8>>,
    buffer_queue_receiver: async_channel::Receiver<Vec<u8>>,
    file_queue_sender: tokio::sync::mpsc::Sender<LoadedFileState>,
    file_queue_receiver: tokio::sync::mpsc::Receiver<LoadedFileState>,
}

impl AssetCache {
    pub fn new(registry: AssetRegistry, dispatcher: Arc<Dispatcher>) -> Self {
        // Create queue of completed file states.
        let (file_sender, file_receiver) =
            tokio::sync::mpsc::channel::<LoadedFileState>(FILE_QUEUE_SIZE);
        // Create queue of buffers
        let (buffers_sender, buffers_receiver) =
            async_channel::bounded::<Vec<u8>>(INITIAL_BUFFER_COUNT);
        // Load buffers into the queue
        (0..INITIAL_BUFFER_COUNT).into_iter().for_each(|_| {
            buffers_sender
                .send_blocking(Vec::with_capacity(INITIAL_BUFFER_SIZE))
                .unwrap();
        });

        AssetCache {
            registry: Arc::new(registry),
            loading_state: Default::default(),
            dispatcher,
            buffer_queue_sender: buffers_sender,
            buffer_queue_receiver: buffers_receiver,
            file_queue_sender: file_sender,
            file_queue_receiver: file_receiver,
        }
    }

    pub fn request_blob(&self, identifier: AssetIdentifier) {
        let descriptor = match self.registry.get_asset_descriptor(identifier) {
            Ok(descriptor) => descriptor,
            Err(e) => {
                t_warn!("Could not get descriptor: {:#?}", e);
                return;
            }
        };
        let registry = Arc::clone(&self.registry);
        let buffer_receiver = self.buffer_queue_receiver.clone();
        let buffer_sender = self.buffer_queue_sender.clone();
        let loaded_sender = self.file_queue_sender.clone();
        self.dispatcher.spawn_async(async move {
            let buffer_receiver = buffer_receiver;
            let mut buffer = match buffer_receiver.try_recv() {
                Ok(buffer) => buffer,
                Err(error) => match error {
                    TryRecvError::Empty => {
                        t_warn!("Empty file buffer queue. Need to block");
                        match buffer_receiver.recv().await {
                            Ok(buffer) => buffer,
                            Err(e) => {
                                t_warn!("{}", e);
                                return;
                            }
                        }
                    }
                    TryRecvError::Closed => {
                        t_warn!("Buffer queue failure!");
                        return;
                    }
                },
            };
            // Resize the buffer to make sure it's correct size.
            buffer.resize(descriptor.file_size() as usize, 0);
            // Start the actual load.
            match registry.load_asset_into(identifier, &mut buffer).await {
                Ok(slice) => {
                    let len = slice.len();
                    let lfs = LoadedFileState {
                        buffer,
                        len,
                        buf_sender: buffer_sender,
                    };
                    match loaded_sender.send(lfs).await {
                        Ok(_) => (),
                        Err(e) => {
                            t_warn!("Could not send loaded file: {:#?}", e);
                        }
                    };
                }
                Err(e) => {}
            };
        });
    }

    pub fn load_typed_into_blocking<'a, 'b, T>(
        &'a self,
        identifier: AssetIdentifier,
        buffer: &'b mut [u8],
    ) -> Result<T, AssetError>
    where
        T: DeserializeOwned,
    {
        let descriptor = match self.registry.get_asset_descriptor(identifier) {
            Ok(d) => d,
            Err(e) => return Err(AssetError::UnknownAssetIdentifier),
        };
        let buf = self.load_blob_into_blocking(identifier, buffer)?;

        match descriptor.format() {
            AssetSerializationFormat::Binary => {
                todo!()
            }
            AssetSerializationFormat::Toml => {
                toml::from_slice(buf).map_err(|e| AssetError::DeserializationFailure)
            }
            AssetSerializationFormat::Yaml => serde_yaml::from_slice(buf).map_err(|e| {
                t_warn!("{e}");
                AssetError::DeserializationFailure
            }),
            AssetSerializationFormat::Unknown => {
                todo!()
            }
        }
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
