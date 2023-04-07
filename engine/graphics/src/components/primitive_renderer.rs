use assets::AssetIdentifier;
use engine::ecs::Component;


#[derive(Component)]
pub struct PrimitiveRenderer {
    mesh: AssetIdentifier,
}

pub struct VkPrimitiveRenderer {}
