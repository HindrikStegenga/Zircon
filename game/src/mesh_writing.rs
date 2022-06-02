use mesh::*;
use std::{fs::File, io::*};

pub fn write_meshes() {
    write_mesh(
        &generate_2d_triangle_in_ndc(),
        "./game/assets/meshes/triangle_2d_ndc.yaml",
    );
}

pub fn write_mesh(mesh: &Mesh, path: &str) {
    let mut file = File::create(path).unwrap();
    let bytes = serde_yaml::to_vec(&mesh).unwrap();
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
}

pub fn generate_2d_triangle_in_ndc() -> Mesh {
    // Vertices are in NDC space.
    let vertex_data: Vec<f32> = vec![
        0.0, -0.5, // top
        0.5, 0.5, // right bottom
        -0.5, 0.5, // left bottom
    ];
    let vertex_data = vertex_data
        .into_iter()
        .flat_map(|f| f.to_ne_bytes())
        .collect::<Vec<_>>();

    let buffers = vec![Buffer::new(vertex_data)];
    let buffer_views = vec![BufferView::new(0, 0, buffers[0].buffer().len() as u32, 8)];
    let accessors = vec![Accessor::new(0, 0, 3, BufferElementFormat::F32x2)];
    let attributes = vec![Attribute::new(0, AttributePurpose::Position2D)];
    let primitives = vec![Primitive::new(
        attributes,
        None,
        PrimitiveRenderingMode::Triangles,
    )];

    let mesh = Mesh::new(buffers, buffer_views, accessors, primitives);

    graphics::vertex_input_layout::extract_input_state_from_mesh(&mesh);

    mesh
}
