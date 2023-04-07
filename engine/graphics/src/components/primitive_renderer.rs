use assets::AssetIdentifier;
use engine::ecs::Component;
use mesh::*;

#[derive(Component)]
pub struct PrimitiveRenderer {
    mesh: AssetIdentifier,
}

pub struct VkPrimitiveRenderer {}
