use engine::ecs::*;
use engine::scene_manager::SceneHandle;

#[derive(Copy, Clone)]
pub struct DidBindCamera {
    scene: SceneHandle,
    entity: Entity,
}
