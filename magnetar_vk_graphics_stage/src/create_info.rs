use crate::config::VkGraphicsOptions;
use magnetar_engine::{
    engine::create_info::ApplicationInfo, resource_system::ResourceSystem, PlatformInterface,
};

#[derive(Debug)]
pub struct VkGraphicsSystemCreateInfo<'a> {
    pub graphics_options: VkGraphicsOptions,
    pub application_info: ApplicationInfo,
    pub render_thread_resources: &'a mut ResourceSystem,
    pub platform_interface: &'a mut dyn PlatformInterface,
}
