use graphyte_engine::ecs::*;
use graphyte_engine::scene_manager::SceneHandle;
use graphyte_engine::Message;

#[derive(Copy, Clone)]
pub struct DidBindCamera {
    scene: SceneHandle,
    entity: Entity,
}
