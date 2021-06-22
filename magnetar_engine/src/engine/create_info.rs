use crate::engine_stages::{RenderStageConstructor, UpdateStageConstructor};

/// Information required to construct an instance of `Engine`.
pub struct EngineCreateInfo {
    pub update_tick_rate: u32,
    pub max_skipped_frames: u32,
    pub max_frame_rate: Option<u32>,
    pub update_stages: Vec<Box<UpdateStageConstructor>>,
    pub render_stages: Vec<Box<RenderStageConstructor>>,
}
