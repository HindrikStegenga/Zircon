use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Accessor {
    buffer_view_index: u32,
    byte_offset: u32,
    element_count: u32,
    format: BufferElementFormat,
}

impl Accessor {
    pub const fn new(
        buffer_view_index: u32,
        byte_offset: u32,
        element_count: u32,
        format: BufferElementFormat,
    ) -> Self {
        Self {
            buffer_view_index,
            byte_offset,
            element_count,
            format,
        }
    }

    /// Get a reference to the accessor's buffer view index.
    pub const fn buffer_view_index(&self) -> u32 {
        self.buffer_view_index
    }

    /// Get a reference to the accessor's byte offset.
    pub const fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    /// Get a reference to the accessor's element count.
    pub const fn element_count(&self) -> u32 {
        self.element_count
    }

    /// Get a reference to the accessor's format.
    pub const fn format(&self) -> BufferElementFormat {
        self.format
    }
}
