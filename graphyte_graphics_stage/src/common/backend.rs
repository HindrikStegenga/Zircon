use std::sync::Arc;
use graphyte_engine::*;
use graphyte_engine::engine_stages::{RenderStageUpdateInput, UpdateStageUpdateInput};
use graphyte_engine::message_bus::MessageRegisterer;
use crate::GraphicsStage;

pub struct GraphicsBackendCreateInfo<'a, T> {
    pub graphics_options: T,
    pub application_info: ApplicationInfo,
    pub asset_system: Arc<AssetSystem>,
    pub platform_interface: &'a mut dyn PlatformInterface,
}

pub trait GraphicsBackend : Sized {
    const API_IDENTIFIER: &'static str;
    type GraphicsOptions;
    type ErrorType;

    fn new(create_info: GraphicsBackendCreateInfo<'_, Self::GraphicsOptions>) -> Result<Self, Self::ErrorType>;

    fn register_message_handlers(&self, mut _registerer: MessageRegisterer<'_, GraphicsStage>) {}
    fn pre_update(input: UpdateStageUpdateInput) -> EngineUpdateResult
    { EngineUpdateResult::Ok }
    fn post_update(input: UpdateStageUpdateInput) -> EngineUpdateResult
    { EngineUpdateResult::Ok }
    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult;
}

