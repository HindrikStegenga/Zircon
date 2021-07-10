use serde::Deserialize;
use std::ffi::CString;

#[derive(Debug, Deserialize)]
pub struct VkGraphicsOptions {
    pub preferred_gpu: Option<CString>,
    pub prefer_integrated_gpu: bool,
    pub prevent_tearing: bool,
    pub limit_frame_rate: bool,
    pub preferred_frames_in_flight: u32,

    pub use_transfer_queues: bool,
    pub requires_platform_swapchain_extensions: bool,

    pub instance_extension_names: Vec<CString>,
    pub instance_extension_names_debug: Vec<CString>,

    pub instance_validation_layer_names: Vec<CString>,
    pub instance_validation_layer_names_debug: Vec<CString>,

    pub default_window_title: String,
    pub default_window_width: u32,
    pub default_window_height: u32,
}
