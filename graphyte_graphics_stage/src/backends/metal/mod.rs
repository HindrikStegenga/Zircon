use graphyte_engine::engine_stages::RenderStageUpdateInput;
use graphyte_engine::EngineUpdateResult;
use crate::{GraphicsBackend, GraphicsBackendCreateInfo};

pub struct MetalRenderBackend {}

impl GraphicsBackend for MetalRenderBackend {
    const API_IDENTIFIER: &'static str = "Metal";
    type GraphicsOptions = ();
    type ErrorType = ();

    fn new(create_info: GraphicsBackendCreateInfo<'_, Self::GraphicsOptions>) -> Result<Self, Self::ErrorType> {
        Ok(Self {})
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}