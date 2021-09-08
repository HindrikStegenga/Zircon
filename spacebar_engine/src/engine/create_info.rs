use std::{ffi::CString, sync::Arc};

use crate::engine_stages::{RenderStageConstructor, UpdateStageConstructor};
use serde::*;
use spacebar_asset_library::asset_system::AssetSystem;

/// Information required to construct an instance of `Engine`.
pub struct EngineCreateInfo {
    pub asset_system: Option<Arc<AssetSystem>>,
    pub application_info: ApplicationInfo,
    pub update_tick_rate: u32,
    pub max_skipped_frames: u32,
    pub max_frame_rate: Option<u32>,
    pub update_stages: Vec<Box<UpdateStageConstructor>>,
    pub render_stages: Vec<Box<RenderStageConstructor>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationInfo {
    pub application_name: CString,
    pub engine_name: CString,

    pub application_major_version: u32,
    pub application_minor_version: u32,
    pub application_patch_version: u32,

    pub engine_major_version: u32,
    pub engine_minor_version: u32,
    pub engine_patch_version: u32,
}
