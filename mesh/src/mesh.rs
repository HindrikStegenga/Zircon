use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Mesh {
    buffers: Vec<Buffer>,
    views: Vec<BufferView>,
    accessors: Vec<Accessor>,
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub const fn new(
        buffers: Vec<Buffer>,
        views: Vec<BufferView>,
        accessors: Vec<Accessor>,
        primitives: Vec<Primitive>,
    ) -> Self {
        Self {
            buffers,
            views,
            accessors,
            primitives,
        }
    }

    /// Get a reference to the mesh's buffers.
    pub fn buffers(&self) -> &[Buffer] {
        self.buffers.as_ref()
    }

    /// Get a reference to the mesh's views.
    pub fn views(&self) -> &[BufferView] {
        self.views.as_ref()
    }

    /// Get a reference to the mesh's accessors.
    pub fn accessors(&self) -> &[Accessor] {
        self.accessors.as_ref()
    }

    /// Get a reference to the mesh's primitives.
    pub fn primitives(&self) -> &[Primitive] {
        self.primitives.as_ref()
    }
}
