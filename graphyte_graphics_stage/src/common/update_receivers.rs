use crate::{CameraIsBoundToWindow, CameraIsUnbound, CameraState, CameraStateUpdate};
use crossbeam::channel::*;
use graphyte_engine::{EngineUpdateResult, RenderStageUpdateInput};
use graphyte_utils::tagged_log;

pub(crate) struct UpdateReceivers {
    pub(crate) cameras_are_updated: Receiver<Vec<CameraStateUpdate>>,
    pub(crate) camera_is_bound: Receiver<CameraIsBoundToWindow>,
    pub(crate) camera_is_unbound: Receiver<CameraIsUnbound>,
}

impl UpdateReceivers {
    pub fn new(
        cameras_are_updated: Receiver<Vec<CameraStateUpdate>>,
        camera_is_bound: Receiver<CameraIsBoundToWindow>,
        camera_is_unbound: Receiver<CameraIsUnbound>,
    ) -> Self {
        UpdateReceivers {
            cameras_are_updated,
            camera_is_bound,
            camera_is_unbound,
        }
    }
}
