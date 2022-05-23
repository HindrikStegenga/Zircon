use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum AttributePurpose {
    Undefined,
    Position2D,
    Position,
    Colors,
    Normals,
    Tangents,
    TexCoords,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Attribute {
    accessor_index: u32,
    purpose: AttributePurpose,
}

impl Attribute {
    pub const fn new(accessor_index: u32, purpose: AttributePurpose) -> Self {
        Self {
            accessor_index,
            purpose,
        }
    }

    /// Get a reference to the attribute's accessor index.
    pub const fn accessor_index(&self) -> u32 {
        self.accessor_index
    }

    /// Get a reference to the attribute's purpose.
    pub const fn purpose(&self) -> AttributePurpose {
        self.purpose
    }
}
