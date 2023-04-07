use ash::vk;
use assets::AssetIdentifier;
use engine::ecs::Component;
use gpu_allocator::vulkan::Allocation;
use mesh::Primitive;

#[derive(Component)]
pub struct PrimitiveRenderer {
    pub id: AssetIdentifier,
    pub primitive: Primitive,
}

pub struct VkPrimitiveRenderer {
    pub vertex_buffers: Vec<vk::Buffer>,
    pub index_buffer: Option<vk::Buffer>,
    pub allocations: Vec<Allocation>,
}
