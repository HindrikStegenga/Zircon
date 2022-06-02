use engine::ecs::Component;
use mesh::*;

#[derive(Component)]
pub struct MeshRenderer {
    mesh: Mesh,
}
