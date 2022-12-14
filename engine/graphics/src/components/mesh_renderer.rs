use assets::AssetIdentifier;
use engine::ecs::Component;
use mesh::*;

#[derive(Component)]
pub struct MeshRenderer {
    mesh: AssetIdentifier,
}

pub struct VulkanMeshRenderer {}
