use crate::engine_stages::{
    AnyRenderStage, AnyRenderStageUpdateThreadHandler, RenderStage, RenderStageUpdateInput,
    RenderStageUpdateThreadHandler, UpdateStageUpdateInput, UpdateThreadHandlerContainer,
};
use crate::message_bus::*;
use crate::{EngineUpdateResult, PlatformInterface};
use std::marker::PhantomData;

pub struct RenderStageContainer<T: RenderStage> {
    stage: T,
    receivers: Vec<Box<dyn AnyRenderMessageReceiver<T>>>,
}

pub struct RenderStageMessageContext<'a> {
    pub platform: &'a mut dyn PlatformInterface,
}

impl<T: RenderStage> From<T> for RenderStageContainer<T> {
    fn from(stage: T) -> Self {
        Self {
            stage,
            receivers: vec![],
        }
    }
}

impl<T: RenderStage> AnyRenderStage for RenderStageContainer<T> {
    fn identifier(&self) -> &'static str {
        <T as RenderStage>::IDENTIFIER
    }

    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>) {
        self.receivers.clear();
        let registerer = RenderMessageRegisterer::new(registerer, &mut self.receivers);
        self.stage.register_message_handlers(registerer);
    }

    fn get_update_thread_handler(
        &mut self,
        registerer: AnyMessageRegisterer<'_>,
    ) -> Box<dyn AnyRenderStageUpdateThreadHandler> {
        let mut item = Box::new(UpdateThreadHandlerContainer::from(
            self.stage.get_update_thread_handler(),
        ));
        item.register_message_handlers(registerer);
        item
    }

    fn process_events(&mut self, input: RenderStageUpdateInput) {
        for receiver in self.receivers.iter_mut() {
            receiver.receive_messages(
                &mut self.stage,
                &mut RenderStageMessageContext {
                    platform: input.platform,
                },
            );
        }
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        self.stage.render(input)
    }
}
