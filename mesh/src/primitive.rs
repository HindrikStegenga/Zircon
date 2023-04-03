use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Primitive {
    /// If mesh requires indexed rendering, a valid index into the accessor array must be provided.
    indices: Option<u32>,
    /// Primitive rendering mode
    mode: PrimitiveRenderingMode,
    /// List of attributes to set up when rendering this primitive.
    attributes: Vec<Attribute>,
}

impl Primitive {
    pub const fn new(
        indices: Option<u32>,
        mode: PrimitiveRenderingMode,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            attributes,
            indices,
            mode,
        }
    }

    /// Get a reference to the primitive's attributes.
    pub fn attributes(&self) -> &[Attribute] {
        self.attributes.as_ref()
    }

    /// Get a reference to the primitive's indices.
    pub const fn indices(&self) -> Option<u32> {
        self.indices
    }
}
