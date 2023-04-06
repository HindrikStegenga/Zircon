use super::data_types::*;

pub const fn buffer_element_format_size(format: BufferElementFormat) -> u32 {
    const U8_SIZE: u32 = core::mem::size_of::<u8>() as u32;
    const I8_SIZE: u32 = core::mem::size_of::<i8>() as u32;

    const U16_SIZE: u32 = core::mem::size_of::<u16>() as u32;
    const I16_SIZE: u32 = core::mem::size_of::<i16>() as u32;

    const U32_SIZE: u32 = core::mem::size_of::<u32>() as u32;
    const I32_SIZE: u32 = core::mem::size_of::<i32>() as u32;

    const U64_SIZE: u32 = core::mem::size_of::<u64>() as u32;
    const I64_SIZE: u32 = core::mem::size_of::<i64>() as u32;

    const F16_SIZE: u32 = core::mem::size_of::<u16>() as u32;
    const F32_SIZE: u32 = core::mem::size_of::<f32>() as u32;
    const F64_SIZE: u32 = core::mem::size_of::<f64>() as u32;

    match format {
        BufferElementFormat::U8x1 => U8_SIZE,
        BufferElementFormat::U8x2 => U8_SIZE * 2,
        BufferElementFormat::U8x3 => U8_SIZE * 3,
        BufferElementFormat::U8x4 => U8_SIZE * 4,

        BufferElementFormat::I8x1 => I8_SIZE,
        BufferElementFormat::I8x2 => I8_SIZE * 2,
        BufferElementFormat::I8x3 => I8_SIZE * 3,
        BufferElementFormat::I8x4 => I8_SIZE * 4,

        BufferElementFormat::U16x1 => U16_SIZE,
        BufferElementFormat::U16x2 => U16_SIZE * 2,
        BufferElementFormat::U16x3 => U16_SIZE * 3,
        BufferElementFormat::U16x4 => U16_SIZE * 4,

        BufferElementFormat::I16x1 => I16_SIZE,
        BufferElementFormat::I16x2 => I16_SIZE * 2,
        BufferElementFormat::I16x3 => I16_SIZE * 3,
        BufferElementFormat::I16x4 => I16_SIZE * 4,

        BufferElementFormat::U32x1 => U32_SIZE,
        BufferElementFormat::U32x2 => U32_SIZE * 2,
        BufferElementFormat::U32x3 => U32_SIZE * 3,
        BufferElementFormat::U32x4 => U32_SIZE * 4,

        BufferElementFormat::I32x1 => I32_SIZE,
        BufferElementFormat::I32x2 => I32_SIZE * 2,
        BufferElementFormat::I32x3 => I32_SIZE * 3,
        BufferElementFormat::I32x4 => I32_SIZE * 4,

        BufferElementFormat::U64x1 => U64_SIZE,
        BufferElementFormat::U64x2 => U64_SIZE * 2,
        BufferElementFormat::U64x3 => U64_SIZE * 3,
        BufferElementFormat::U64x4 => U64_SIZE * 4,

        BufferElementFormat::I64x1 => I64_SIZE,
        BufferElementFormat::I64x2 => I64_SIZE * 2,
        BufferElementFormat::I64x3 => I64_SIZE * 3,
        BufferElementFormat::I64x4 => I64_SIZE * 4,

        BufferElementFormat::F16x1 => F16_SIZE,
        BufferElementFormat::F16x2 => F16_SIZE * 2,
        BufferElementFormat::F16x3 => F16_SIZE * 3,
        BufferElementFormat::F16x4 => F16_SIZE * 4,

        BufferElementFormat::F32x1 => F32_SIZE,
        BufferElementFormat::F32x2 => F32_SIZE * 2,
        BufferElementFormat::F32x3 => F32_SIZE * 3,
        BufferElementFormat::F32x4 => F32_SIZE * 4,

        BufferElementFormat::F64x1 => F64_SIZE,
        BufferElementFormat::F64x2 => F64_SIZE * 2,
        BufferElementFormat::F64x3 => F64_SIZE * 3,
        BufferElementFormat::F64x4 => F64_SIZE * 4,
    }
}
