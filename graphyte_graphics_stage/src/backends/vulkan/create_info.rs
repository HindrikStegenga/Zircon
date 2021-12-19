use std::sync::Arc;

use super::config::*;
use graphyte_engine::{engine::create_info::ApplicationInfo, AssetSystem, PlatformInterface};

pub struct VkGraphicsSystemCreateInfo<'a> {
    pub graphics_options: VkGraphicsOptions,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetSystem>,
    pub platform_interface: &'a mut dyn PlatformInterface,
}
