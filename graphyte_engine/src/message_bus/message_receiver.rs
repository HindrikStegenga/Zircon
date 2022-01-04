use super::*;
use crossbeam::channel::*;
use std::marker::PhantomData;
use crate::engine_stages::{RenderStageMessageContext, UpdateStageMessageContext};

pub struct MessageReceiver<C, M: Message, T: MessageHandler<C, M>> {
    receiver: Receiver<M>,
    _phantom: PhantomData<fn(C, T)>,
}

impl<C, M: Message, T: MessageHandler<C, M>> MessageReceiver<C, M, T> {
    pub fn new(receiver: Receiver<M>) -> Self {
        Self {
            receiver,
            _phantom: Default::default(),
        }
    }
}

pub trait AnyRenderMessageReceiver<T>: Send {
    fn receive_messages<'a>(&'a mut self, receiver: &'a mut T, context: &'a mut RenderStageMessageContext<'a>);
}

pub trait AnyUpdateMessageReceiver<T>: Send {
    fn receive_messages<'a>(&'a mut self, receiver: &'a mut T, context: &'a mut UpdateStageMessageContext<'a>);
}

impl<M: Message, T: for<'a> MessageHandler<RenderStageMessageContext<'a>, M>> AnyRenderMessageReceiver<T> for MessageReceiver<RenderStageMessageContext<'_>, M, T> {
    fn receive_messages<'b>(&'b mut self, receiver: &'b mut T, context: &'b mut RenderStageMessageContext<'b>) {
        for message in self.receiver.try_iter() {
            receiver.handle(context, message);
        }
    }
}

impl<M: Message, T: for<'a> MessageHandler<UpdateStageMessageContext<'a>, M>> AnyUpdateMessageReceiver<T> for MessageReceiver<UpdateStageMessageContext<'_>, M, T> {
    fn receive_messages<'b>(&'b mut self, receiver: &'b mut T, context: &'b mut UpdateStageMessageContext<'b>) {
        for message in self.receiver.try_iter() {
            receiver.handle(context, message);
        }
    }
}
