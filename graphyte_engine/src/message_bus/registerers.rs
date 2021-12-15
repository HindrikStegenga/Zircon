use std::marker::PhantomData;
use crossbeam::channel::*;
use super::*;

pub struct AnyMessageRegisterer<'a> {
    builder: &'a mut MessageBusBuilder,
    handler_type: MessageHandlerType,
}


impl<'a> AnyMessageRegisterer<'a> {
    pub fn new(builder: &'a mut MessageBusBuilder, handler_type: MessageHandlerType) -> Self {
        Self {
            builder,
            handler_type
        }
    }

    pub fn register<M: Message>(&mut self) -> Receiver<M> {
        self.builder.add_update_handler::<M>(self.handler_type)
    }
}

pub struct MessageRegisterer<'a, T: 'static> {
    registerer: AnyMessageRegisterer<'a>,
    receivers: &'a mut Vec<Box<dyn AnyMessageReceiver<T>>>,
    _phantom: PhantomData<fn(T)>
}

impl<'a, T: 'static> MessageRegisterer<'a, T> {

    pub fn new(registerer: AnyMessageRegisterer<'a>, receivers: &'a mut Vec<Box<dyn AnyMessageReceiver<T>>>) -> Self {
        Self {
            registerer,
            receivers,
            _phantom: Default::default()
        }
    }

    pub fn register<M: Message>(&mut self) where T : MessageHandler<M> {
        let receiver = self.registerer.register::<M>();
        self.receivers.push(Box::from(MessageReceiver::<M, T>::new(receiver)));
    }
}