use crate::common::update_receivers::UpdateReceivers;
use crate::{CameraManager};
use crossbeam::channel::*;
use engine::engine_stages::{RenderStageUpdateThreadHandler};
use engine::resource_manager::ThreadLocalResourceManager;
use engine::{
    EngineUpdateResult, UpdateMessageRegisterer, UpdateStageUpdateInput,
};


pub struct GraphicsStageUpdateThreadHandler {}

impl GraphicsStageUpdateThreadHandler {
    pub(crate) fn new(resources: &mut ThreadLocalResourceManager) -> (Self, UpdateReceivers) {
        let (cameras_updated_sender, cameras_updated_receiver) = unbounded();
        let (camera_is_bound_sender, camera_is_bound_receiver) = unbounded();
        let (camera_is_unbound_sender, camera_is_unbound_receiver) = unbounded();

        resources.add_resource(CameraManager::new(
            cameras_updated_sender,
            camera_is_bound_sender,
            camera_is_unbound_sender,
        ));
        let handler = Self {};
        let receiver = UpdateReceivers::new(
            cameras_updated_receiver,
            camera_is_bound_receiver,
            camera_is_unbound_receiver,
        );
        (handler, receiver)
    }
}

impl RenderStageUpdateThreadHandler for GraphicsStageUpdateThreadHandler {
    fn register_message_handlers(&self, _registerer: UpdateMessageRegisterer<'_, Self>) {}

    fn post_update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        let camera_manager = match input
            .update_thread_resources
            .get_resource_mut::<CameraManager>()
        {
            Some(v) => v,
            None => return EngineUpdateResult::Stop,
        };
        camera_manager.update_cameras(input.scene_manager.active_scene_mut().registry_mut());
        EngineUpdateResult::Ok
    }
}
