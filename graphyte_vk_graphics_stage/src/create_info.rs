use std::sync::Arc;

use crate::config::VkGraphicsOptions;
use graphyte_engine::{
    engine::create_info::ApplicationInfo, resource_system::ResourceSystem, AssetSystem,
    PlatformInterface,
};

pub struct VkGraphicsSystemCreateInfo<'a> {
    pub graphics_options: VkGraphicsOptions,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetSystem>,
    pub render_thread_resources: &'a mut ResourceSystem,
    pub platform_interface: &'a mut dyn PlatformInterface,
}
