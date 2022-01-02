use super::*;
use std::sync::Arc;
use graphyte_engine::*;

pub struct GraphicsStageCreateInfo<'a> {
    pub platform: &'a mut dyn PlatformInterface,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetSystem>,
    pub options: GraphicsOptions
}