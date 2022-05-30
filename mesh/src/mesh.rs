use crate::*;
use itertools::*;
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

    pub fn accessors_for(&self, primitive: &Primitive) -> Vec<&Accessor> {
        primitive
            .attributes()
            .into_iter()
            .map(|a| a.accessor_index())
            .unique()
            .map(|i| &self.accessors[i as usize])
            .collect()
    }

    pub fn buffer_views_for(&self, primitive: &Primitive) -> Vec<&BufferView> {
        let accessors = self.accessors_for(primitive);
        accessors
            .into_iter()
            .map(|a| a.buffer_view_index())
            .unique()
            .map(|i| &self.views()[i as usize])
            .collect()
    }

    pub fn buffers_for(&self, primitive: &Primitive) -> Vec<&Buffer> {
        let buffers = self.buffer_views_for(primitive);
        buffers
            .into_iter()
            .map(|bv| bv.buffer_index())
            .unique()
            .map(|i| &self.buffers()[i as usize])
            .collect()
    }
}
