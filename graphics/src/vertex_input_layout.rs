use mesh::*;

#[repr(u8)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VertexInputBinding {
    binding: u32,
    stride: u32,
    input_rate: VertexInputRate,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VertexInputAttribute {
    binding: u32,
    location: u32,
    format: BufferElementFormat,
    offset: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VertexInputLayout {
    bindings: Vec<VertexInputBinding>,
    attributes: Vec<VertexInputAttribute>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VertexInputState {
    topology: mesh::PrimitiveRenderingMode,
    primitive_restart_enable: bool,
    layout: VertexInputLayout,
}

pub fn extract_input_state_from_mesh(mesh: &Mesh) -> Option<VertexInputState> {
    if mesh.primitives().len() > 1 {
        todo!();
    } //TODO Support this

    for primitive in mesh.primitives() {
        let views = mesh.buffer_views_for(primitive);
        // Check if we have indexed rendering.
        if let Some(_indices_accessor_index) = primitive.indices() {
            todo!();
        } else {
            // No indexed rendering required.
            let bindings = views
                .into_iter()
                .enumerate()
                .map(|(idx, (bv, bv_idx))| {
                    (
                        VertexInputBinding {
                            binding: idx as u32,
                            stride: bv.byte_stride(),
                            input_rate: VertexInputRate::Vertex,
                        },
                        (bv, bv_idx),
                    )
                })
                .collect::<Vec<_>>();

            let mut input_attributes = vec![];
            bindings
                .iter()
                .enumerate()
                .for_each(|(binding_idx, (_binding, (_bv, bv_idx)))| {
                    // Get all the bindings associated with this
                    let mut accessors_for_buffer_view =
                        mesh.accessors_for_buffer_view(primitive, *bv_idx);
                    accessors_for_buffer_view.sort_by_key(|e| e.0.byte_offset());

                    accessors_for_buffer_view
                        .iter()
                        .enumerate()
                        .for_each(|(location, (a, _i))| {
                            input_attributes.push(VertexInputAttribute {
                                binding: binding_idx as u32,
                                location: location as u32,
                                format: a.format(),
                                offset: a.byte_offset(),
                            });
                        });
                });
        }
    }

    None
}
