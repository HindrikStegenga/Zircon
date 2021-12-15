use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use crossbeam::channel::{Receiver, Sender, unbounded};
use crate::message_bus::{Message, MessageHandler, MessageRegisterer};
use crate::message_bus::message_sender::MessageSender;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MessageHandlerType {
    Render = 0,
    Update = 1
}

pub struct MessageBusBuilder {
    channels: HashMap<TypeId, Box<dyn Any + Sync + Send>>,
    finalization_handlers: Vec<fn(map: &mut HashMap<TypeId, Box<dyn Any + Sync + Send>>)>
}

impl Default for MessageBusBuilder {
    fn default() -> Self {
        Self { channels: Default::default(), finalization_handlers: vec![] }
    }
}

impl MessageBusBuilder {

    fn finalize<M: Message>(map: &mut HashMap<TypeId, Box<dyn Any + Send + Sync>>) {
        let value = map.remove(&TypeId::of::<M>()).unwrap();
        let (render, update) : (Vec<Sender<M>>, Vec<Sender<M>>) = *(value.downcast::<(Vec<Sender<M>>, Vec<Sender<M>>)>().unwrap());
        let values : Arc<(Vec<Sender<M>>, Vec<Sender<M>>)> = Arc::new((render, update));
        map.insert(TypeId::of::<M>(), Box::from(MessageSender::new(values)));
    }

    fn get_senders_mut<M: Message>(&mut self, handler_type: MessageHandlerType) -> Option<&mut Vec<Sender<M>>> {
        return if let Some(value) = self.channels.get_mut(&TypeId::of::<M>()) {
            let (render_senders, update_senders) = value.downcast_mut::<(Vec<Sender<M>>, Vec<Sender<M>>)>().unwrap();
            match handler_type {
                MessageHandlerType::Render => {
                    Some(render_senders)
                }
                MessageHandlerType::Update => {
                    Some(update_senders)
                }
            }
        } else { None }
    }

    pub fn add_update_handler<M: Message>(&mut self, handler_type: MessageHandlerType) -> Receiver<M> {
        return match self.get_senders_mut::<M>(handler_type) {
            Some(values) => {
                let (sender, receiver) = unbounded();
                values.push(sender);
                receiver
            },
            None => {
                self.channels.insert(TypeId::of::<M>(), Box::from((Vec::<Sender<M>>::new(), Vec::<Sender<M>>::new())));
                self.finalization_handlers.push(Self::finalize::<M>);
                let values = self.get_senders_mut(handler_type).unwrap();
                let (sender, receiver) = unbounded();
                values.push(sender);
                receiver
            }
        }
    }

    pub fn build(mut self) -> MessageBus {
        for handler in self.finalization_handlers {
            (handler)(&mut self.channels);
        }
        MessageBus { channels: self.channels }
    }
}


pub struct MessageBus {
    channels: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl MessageBus {
    pub fn get_sender<M: Message>(&self) -> Option<MessageSender<M>> {
        return if let Some(v) = self.channels.get(&TypeId::of::<M>()) {
            let sender = v.downcast_ref::<MessageSender<M>>().unwrap();
            Some(sender.clone())
        } else { None }
    }
}