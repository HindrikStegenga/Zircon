use ash::vk;
use mesh::*;

const fn to_vk_input_rate(input_rate: InputRate) -> vk::VertexInputRate {
    match input_rate {
        InputRate::PerVertex => vk::VertexInputRate::VERTEX,
        InputRate::PerInstance => vk::VertexInputRate::INSTANCE,
    }
}

const fn to_vk_format(format: BufferElementFormat) -> vk::Format {
    match format {
        BufferElementFormat::U8x1 => vk::Format::R8_UINT,
        BufferElementFormat::U8x2 => vk::Format::R8G8_UINT,
        BufferElementFormat::U8x3 => vk::Format::R8G8B8_UINT,
        BufferElementFormat::U8x4 => vk::Format::R8G8B8A8_UINT,

        BufferElementFormat::I8x1 => vk::Format::R8_SINT,
        BufferElementFormat::I8x2 => vk::Format::R8G8_SINT,
        BufferElementFormat::I8x3 => vk::Format::R8G8B8_SINT,
        BufferElementFormat::I8x4 => vk::Format::R8G8B8A8_SINT,

        BufferElementFormat::U16x1 => vk::Format::R16_UINT,
        BufferElementFormat::U16x2 => vk::Format::R16G16_UINT,
        BufferElementFormat::U16x3 => vk::Format::R16G16B16_UINT,
        BufferElementFormat::U16x4 => vk::Format::R16G16B16A16_UINT,

        BufferElementFormat::I16x1 => vk::Format::R16_SINT,
        BufferElementFormat::I16x2 => vk::Format::R16G16_SINT,
        BufferElementFormat::I16x3 => vk::Format::R16G16B16_SINT,
        BufferElementFormat::I16x4 => vk::Format::R16G16B16A16_SINT,

        BufferElementFormat::U32x1 => vk::Format::R32_UINT,
        BufferElementFormat::U32x2 => vk::Format::R32G32_UINT,
        BufferElementFormat::U32x3 => vk::Format::R32G32B32_UINT,
        BufferElementFormat::U32x4 => vk::Format::R32G32B32A32_UINT,

        BufferElementFormat::I32x1 => vk::Format::R32_SINT,
        BufferElementFormat::I32x2 => vk::Format::R32G32_SINT,
        BufferElementFormat::I32x3 => vk::Format::R32G32B32_SINT,
        BufferElementFormat::I32x4 => vk::Format::R32G32B32A32_SINT,

        BufferElementFormat::U64x1 => vk::Format::R64_UINT,
        BufferElementFormat::U64x2 => vk::Format::R64G64_UINT,
        BufferElementFormat::U64x3 => vk::Format::R64G64B64_UINT,
        BufferElementFormat::U64x4 => vk::Format::R64G64B64A64_UINT,

        BufferElementFormat::I64x1 => vk::Format::R64_SINT,
        BufferElementFormat::I64x2 => vk::Format::R64G64_SINT,
        BufferElementFormat::I64x3 => vk::Format::R64G64B64_SINT,
        BufferElementFormat::I64x4 => vk::Format::R64G64B64A64_SINT,

        BufferElementFormat::F16x1 => vk::Format::R16_SFLOAT,
        BufferElementFormat::F16x2 => vk::Format::R16G16_SFLOAT,
        BufferElementFormat::F16x3 => vk::Format::R16G16B16_SFLOAT,
        BufferElementFormat::F16x4 => vk::Format::R16G16B16A16_SFLOAT,

        BufferElementFormat::F32x1 => vk::Format::R32_SFLOAT,
        BufferElementFormat::F32x2 => vk::Format::R32G32_SFLOAT,
        BufferElementFormat::F32x3 => vk::Format::R32G32B32_SFLOAT,
        BufferElementFormat::F32x4 => vk::Format::R32G32B32A32_SFLOAT,

        BufferElementFormat::F64x1 => vk::Format::R64_SFLOAT,
        BufferElementFormat::F64x2 => vk::Format::R64G64_SFLOAT,
        BufferElementFormat::F64x3 => vk::Format::R64G64B64_SFLOAT,
        BufferElementFormat::F64x4 => vk::Format::R64G64B64A64_SFLOAT,
    }
}

fn get_bindings_and_attributes(
    primitive: &Primitive,
) -> (
    Vec<vk::VertexInputBindingDescription>,
    Vec<vk::VertexInputAttributeDescription>,
) {
    let mut bindings = vec![];
    let mut attributes = vec![];

    let mut push_attributes = |binding: &BufferBinding, binding_index: u32| {
        for attribute in &binding.attributes {
            attributes.push(
                vk::VertexInputAttributeDescription::builder()
                    .binding(binding_index as u32)
                    .format(to_vk_format(attribute.format))
                    .location(attribute.location)
                    .offset(attribute.offset_in_bytes as u32)
                    .build(),
            );
        }
    };

    // Generate binding and attributes for all non index buffer purposes.
    for binding_index in 0..primitive.bindings.len() {
        let binding = &primitive.bindings[binding_index];
        bindings.push(
            vk::VertexInputBindingDescription::builder()
                .binding(binding_index as u32)
                .input_rate(to_vk_input_rate(binding.input_rate))
                .stride(binding.stride_in_bytes)
                .build(),
        );
        push_attributes(binding, binding_index as u32);
    }

    // Generate binding and attributes for index buffer purposes.
    if let Some(binding) = &primitive.index_buffer_binding {
        bindings.push(
            vk::VertexInputBindingDescription::builder()
                .binding(primitive.bindings.len() as u32)
                .input_rate(to_vk_input_rate(binding.input_rate))
                .stride(binding.stride_in_bytes)
                .build(),
        );
        push_attributes(binding, primitive.bindings.len() as u32);
    }

    (bindings, attributes)
}
