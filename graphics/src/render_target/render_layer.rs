use super::AcquiredFrameInfo;

pub struct RenderLayerRenderInfo<'a> {
    info: &'a AcquiredFrameInfo,
}

impl<'a> RenderLayerRenderInfo<'a> {}

impl<'a> RenderLayerRenderInfo<'a> {
    pub(crate) fn new(info: &'a AcquiredFrameInfo) -> Self {
        Self { info }
    }
}

pub trait RenderLayer {
    fn swapchain_will_be_resized(&mut self);
    fn swapchain_did_resize(&mut self);
    fn pre_render(&mut self, info: RenderLayerRenderInfo);
    fn post_render(&mut self, info: RenderLayerRenderInfo);
}
