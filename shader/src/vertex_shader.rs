use mesh::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VertexShaderDescriptor {
    input_layout: VertexShaderInputLayout,
    source_bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct VertexShaderInputLayout {
    attributes: Vec<InputAttributeDescription>,
    stride: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InputAttributeDescription {
    location: u32,
    data_type: BufferElementFormat,
    purpose: AttributePurpose,
}
