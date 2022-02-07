use crate::{CameraIsBoundToWindow, CameraIsUnbound, CameraState, CameraStateUpdate};
use crossbeam::channel::*;
use graphyte_engine::{EngineUpdateResult, RenderStageUpdateInput};

pub(crate) struct UpdateReceiver {
    cameras_are_updated: Receiver<Vec<CameraStateUpdate>>,
    camera_is_bound: Receiver<CameraIsBoundToWindow>,
    camera_is_unbound: Receiver<CameraIsUnbound>,
}

impl UpdateReceiver {
    pub fn new(
        cameras_are_updated: Receiver<Vec<CameraStateUpdate>>,
        camera_is_bound: Receiver<CameraIsBoundToWindow>,
        camera_is_unbound: Receiver<CameraIsUnbound>,
    ) -> Self {
        UpdateReceiver {
            cameras_are_updated,
            camera_is_bound,
            camera_is_unbound,
        }
    }

    pub fn receive(&mut self, input: &mut RenderStageUpdateInput) -> EngineUpdateResult {
        let _input = input;
        while let Ok(_message) = self.camera_is_unbound.try_recv() {}
        while let Ok(_message) = self.camera_is_bound.try_recv() {}
        while let Ok(_message) = self.cameras_are_updated.try_recv() {}
        EngineUpdateResult::Ok
    }
}
