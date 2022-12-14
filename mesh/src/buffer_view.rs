use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Buffer {
    buffer: Vec<u8>,
}

impl Buffer {
    pub const fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    /// Get a reference to the buffer's buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl<'a> From<&'a Buffer> for &'a [u8] {
    fn from(buffer: &'a Buffer) -> Self {
        buffer.buffer()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BufferView {
    buffer_index: u32,
    byte_offset: u32,
    byte_length: u32,
    /// The byte stride is the size of each element in the buffer.
    /// Where element is defined by the sum of accessor sizes.
    /// (vec2f, vec2f) => byte_stride = 8.
    /// (vec3f) => byte_stride = 4.
    byte_stride: Option<u32>,
}

impl BufferView {
    pub const fn new(
        buffer_index: u32,
        byte_offset: u32,
        byte_length: u32,
        byte_stride: Option<u32>,
    ) -> Self {
        Self {
            buffer_index,
            byte_offset,
            byte_length,
            byte_stride,
        }
    }

    /// Get a reference to the buffer view's buffer index.
    pub const fn buffer_index(&self) -> u32 {
        self.buffer_index
    }

    /// Get a reference to the buffer view's byte offset.
    pub const fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    /// Get a reference to the buffer view's byte length.
    pub const fn byte_length(&self) -> u32 {
        self.byte_length
    }

    /// Get a reference to the buffer view's byte stride. (Element size)
    pub const fn byte_stride(&self) -> Option<u32> {
        self.byte_stride
    }
}
