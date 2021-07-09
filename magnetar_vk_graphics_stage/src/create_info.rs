use crate::config::VkGraphicsOptions;
use magnetar_engine::engine::create_info::ApplicationInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VkGraphicsSystemCreateInfo {
    pub graphics_options: VkGraphicsOptions,
    pub application_info: ApplicationInfo,
}
