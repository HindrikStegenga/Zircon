use ash::*;
use mesh::*;

fn extract_vertex_attribute_descriptions(mesh: &Mesh) -> Vec<vk::VertexInputAttributeDescription> {
    let mut descriptions = vec![];

    for primitive in mesh.primitives() {
        for attribute in primitive.attributes() {
            let accessor = &mesh.accessors()[attribute.accessor_index() as usize];
            let buffer_view = &mesh.views()[accessor.buffer_view_index() as usize];
        }
    }

    descriptions
}
