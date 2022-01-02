use std::ffi::CString;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GraphicsOptions {
    pub vk_api_major_version: u32,
    pub vk_api_minor_version: u32,
    pub vk_api_patch_version: u32,
    pub instance_extension_names: Vec<CString>,
    pub instance_validation_layer_names: Vec<CString>,
    pub preferred_device_name: Option<CString>,
    pub prefer_integrated_gpu: bool,
    pub prevent_tearing: bool,
    pub limit_frame_rate: bool,
    pub preferred_frames_in_flight: u32,
    pub use_transfer_queues: bool,
}