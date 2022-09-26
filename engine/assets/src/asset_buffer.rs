use crossbeam::queue::ArrayQueue;
use std::sync::Arc;
use utils::t_warn;

/// Represents a borrowed buffer that has valid file contents stored inside.
/// The buffer data inside can only be immutably borrowed.
/// On dropping this object, the buffer will be recycled and send back to the asset cache.
/// In case this fails, the buffer is dropped instead and a warning logged.
#[derive(Debug)]
pub struct AssetBuffer {
    buffer: Vec<u8>,
    used_byte_count: usize,
    available_buffers: Arc<ArrayQueue<Vec<u8>>>,
}

impl AssetBuffer {
    pub(crate) fn new(
        buffer: Vec<u8>,
        used_byte_count: usize,
        available_buffers: Arc<ArrayQueue<Vec<u8>>>,
    ) -> Self {
        Self {
            buffer,
            used_byte_count,
            available_buffers,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer[0..self.used_byte_count]
    }
}

impl AsRef<[u8]> for AssetBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.buffer[0..self.used_byte_count]
    }
}

impl Drop for AssetBuffer {
    fn drop(&mut self) {
        let buf = std::mem::take(&mut self.buffer);
        let byte_count = buf.capacity();
        if let Err(_) = self.available_buffers.push(buf) {
            t_warn!(
                "Could not make buffer available. Dropping buffer: {} bytes.",
                byte_count
            );
        }
    }
}
