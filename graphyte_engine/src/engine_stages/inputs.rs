use crate::resource_manager::{EngineResourceManager, ThreadLocalResourceManager};
use crate::scene_manager::SceneManager;
use crate::PlatformInterface;
use graphyte_utils::dispatcher::Dispatcher;
use std::sync::Arc;

pub struct PlatformPreDidInitInput<'a> {
    pub resources: Arc<EngineResourceManager>,
    pub scene_manager: &'a mut SceneManager,
    pub update_thread_resources: &'a mut ThreadLocalResourceManager,
    pub dispatcher: Arc<Dispatcher>,
}

pub struct EngineDidInitInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub resources: Arc<EngineResourceManager>,
    pub scene_manager: &'a mut SceneManager,
    pub update_thread_resources: &'a mut ThreadLocalResourceManager,
    pub dispatcher: Arc<Dispatcher>,
}

pub struct RenderStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub resources: Arc<EngineResourceManager>,
}

impl<'a> RenderStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        resources: Arc<EngineResourceManager>,
    ) -> Self {
        RenderStageConstructorInput {
            platform_interface,
            resources,
        }
    }
}

pub struct RenderStageUpdateInput<'a> {
    pub platform: &'a mut dyn PlatformInterface,
}

impl<'a> RenderStageUpdateInput<'a> {
    pub fn new(platform: &'a mut dyn PlatformInterface) -> Self {
        Self { platform }
    }
}

pub struct UpdateStageConstructorInput<'a> {
    pub platform_interface: &'a mut dyn PlatformInterface,
    pub resources: Arc<EngineResourceManager>,
}

impl<'a> UpdateStageConstructorInput<'a> {
    pub fn new(
        platform_interface: &'a mut dyn PlatformInterface,
        resources: Arc<EngineResourceManager>,
    ) -> Self {
        Self {
            platform_interface,
            resources,
        }
    }
}

pub struct UpdateStageUpdateInput<'a> {
    pub scene_manager: &'a mut SceneManager,
    pub resources: Arc<EngineResourceManager>,
    pub update_thread_resources: &'a mut ThreadLocalResourceManager,
    pub dispatcher: Arc<Dispatcher>,
}

impl<'a> UpdateStageUpdateInput<'a> {
    pub fn new(
        resources: Arc<EngineResourceManager>,
        dispatcher: Arc<Dispatcher>,
        scene_manager: &'a mut SceneManager,
        thread_local_resources: &'a mut ThreadLocalResourceManager,
    ) -> Self {
        Self {
            scene_manager,
            resources,
            update_thread_resources: thread_local_resources,
            dispatcher,
        }
    }
}
