use super::*;
use crossbeam::channel::*;
use std::marker::PhantomData;

pub struct MessageReceiver<M: Message, T: MessageHandler<M>> {
    receiver: Receiver<M>,
    _phantom: PhantomData<fn(T)>,
}

impl<M: Message, T: MessageHandler<M>> MessageReceiver<M, T> {
    pub fn new(receiver: Receiver<M>) -> Self {
        Self {
            receiver,
            _phantom: Default::default(),
        }
    }
}

pub trait AnyMessageReceiver<T>: Send {
    fn receive_messages(&mut self, receiver: &mut T);
}

impl<M: Message, T: MessageHandler<M>> AnyMessageReceiver<T> for MessageReceiver<M, T> {
    fn receive_messages(&mut self, receiver: &mut T) {
        for message in self.receiver.try_iter() {
            receiver.handle(message);
        }
    }
}
