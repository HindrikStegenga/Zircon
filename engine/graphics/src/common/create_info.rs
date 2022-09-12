use super::*;
use assets::*;
use engine::*;
use std::sync::Arc;

pub struct GraphicsStageCreateInfo<'a> {
    pub platform: &'a mut dyn PlatformInterface,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetCache>,
    pub options: GraphicsOptions,
}
