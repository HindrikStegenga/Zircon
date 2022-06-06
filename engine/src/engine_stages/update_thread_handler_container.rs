use crate::engine_stages::*;
use crate::*;

pub struct UpdateThreadHandlerContainer<T: RenderStageUpdateThreadHandler> {
    stage: T,
    receivers: Vec<Box<dyn AnyUpdateMessageReceiver<T>>>,
}

impl<T: RenderStageUpdateThreadHandler> From<T> for UpdateThreadHandlerContainer<T> {
    fn from(stage: T) -> Self {
        Self {
            stage,
            receivers: vec![],
        }
    }
}

impl<T> AnyRenderStageUpdateThreadHandler for UpdateThreadHandlerContainer<T>
where
    T: RenderStageUpdateThreadHandler,
{
    fn register_message_handlers(&mut self, registerer: AnyMessageRegisterer<'_>) {
        self.receivers.clear();
        let registerer = UpdateMessageRegisterer::new(registerer, &mut self.receivers);
        self.stage.register_message_handlers(registerer);
    }

    fn process_events(&mut self, _input: UpdateStageUpdateInput) {
        for receiver in self.receivers.iter_mut() {
            receiver.receive_messages(&mut self.stage, &mut UpdateStageMessageContext::default());
        }
    }

    fn pre_update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.pre_update(input)
    }

    fn post_update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        self.stage.post_update(input)
    }
}
