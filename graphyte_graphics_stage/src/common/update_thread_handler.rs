use crate::common::*;
use crate::{CameraManager, CameraStatesUpdate};
use graphyte_engine::engine_stages::{RenderStageUpdateThreadHandler, UpdateStageMessageContext};
use graphyte_engine::{
    EngineUpdateResult, MessageHandler, UpdateMessageRegisterer, UpdateStageUpdateInput,
    WindowDidOpen, WindowDidResize,
};
use graphyte_utils::*;
use std::sync::mpsc::Sender;

pub struct GraphicsStageUpdateThreadHandler {
    cameras_updated_sender: Sender<CameraStatesUpdate>,
}

impl GraphicsStageUpdateThreadHandler {
    pub(crate) fn new(cameras_updated_sender: Sender<CameraStatesUpdate>) -> Self {
        Self {
            cameras_updated_sender,
        }
    }
}

impl RenderStageUpdateThreadHandler for GraphicsStageUpdateThreadHandler {
    fn register_message_handlers(&self, mut registerer: UpdateMessageRegisterer<'_, Self>) {}

    fn pre_update(&mut self, mut input: UpdateStageUpdateInput) -> EngineUpdateResult {
        let camera_manager = match input.thread_local_resources().get_resource_mut::<CameraManager>() {
            Some(v) => v,
            None => return EngineUpdateResult::Stop,
        };

        EngineUpdateResult::Ok
    }
}