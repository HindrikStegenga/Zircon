use crate::common::update_receivers::UpdateReceivers;
use crate::common::*;
use crate::{CameraIsBoundToWindow, CameraIsUnbound, CameraManager, CameraStateUpdate};
use crossbeam::channel::*;
use engine::engine_stages::{RenderStageUpdateThreadHandler, UpdateStageMessageContext};
use engine::resource_manager::ThreadLocalResourceManager;
use engine::{
    EngineUpdateResult, MessageHandler, UpdateMessageRegisterer, UpdateStageUpdateInput,
    WindowDidOpen, WindowDidResize,
};
use utils::*;

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
    fn register_message_handlers(&self, mut registerer: UpdateMessageRegisterer<'_, Self>) {}

    fn post_update(&mut self, mut input: UpdateStageUpdateInput) -> EngineUpdateResult {
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
