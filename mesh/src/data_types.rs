use serde::*;

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum RenderingMode {
    /// Corresponds to `GL_POINTS`.
    Points = 1,

    /// Corresponds to `GL_LINES`.
    Lines,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum InputRate {
    PerVertex = 0,
    PerInstance = 1,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum BufferElementFormat {
    U8x1 = 0,
    U8x2,
    U8x3,
    U8x4,

    I8x1,
    I8x2,
    I8x3,
    I8x4,

    U16x1,
    U16x2,
    U16x3,
    U16x4,

    I16x1,
    I16x2,
    I16x3,
    I16x4,

    U32x1,
    U32x2,
    U32x3,
    U32x4,

    I32x1,
    I32x2,
    I32x3,
    I32x4,

    U64x1,
    U64x2,
    U64x3,
    U64x4,

    I64x1,
    I64x2,
    I64x3,
    I64x4,

    F16x1,
    F16x2,
    F16x3,
    F16x4,

    F32x1,
    F32x2,
    F32x3,
    F32x4,

    F64x1,
    F64x2,
    F64x3,
    F64x4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BufferAttribute {
    pub location: u32,
    pub offset_in_bytes: u32,
    pub format: BufferElementFormat,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BufferBinding {
    pub attributes: Vec<BufferAttribute>,
    pub buffer_index: u32,
    pub stride_in_bytes: u32,
    pub input_rate: InputRate,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Primitive {
    pub rendering_mode: RenderingMode,
    pub buffers: Vec<Vec<u8>>,
    pub bindings: Vec<BufferBinding>,
    pub index_buffer_binding: Option<BufferBinding>,
}

pub type Mesh = Vec<Primitive>;
