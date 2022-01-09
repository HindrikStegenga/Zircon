use crate::common::*;
use crate::CameraStatesUpdate;
use graphyte_engine::engine_stages::{RenderStageUpdateThreadHandler, UpdateStageMessageContext};
use graphyte_engine::{
    EngineUpdateResult, MessageHandler, UpdateMessageRegisterer, UpdateStageUpdateInput,
    WindowDidOpen, WindowDidResize,
};
use graphyte_utils::tagged_log;
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
    fn register_message_handlers(&self, mut registerer: UpdateMessageRegisterer<'_, Self>) {
        registerer.register::<WindowDidResize>();
    }
}

impl<'a> MessageHandler<UpdateStageMessageContext<'a>, WindowDidResize>
    for GraphicsStageUpdateThreadHandler
{
    fn handle(&mut self, context: &mut UpdateStageMessageContext<'a>, message: WindowDidResize) {
        tagged_log!("Graphics: Update", "Resize!")
    }
}
