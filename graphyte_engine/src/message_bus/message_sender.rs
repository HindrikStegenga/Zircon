use super::*;
use crossbeam::channel::Sender;
use std::sync::Arc;

#[derive(Debug)]
pub struct MessageSender<M: Message> {
    senders: Arc<(Vec<Sender<M>>, Vec<Sender<M>>)>,
}

impl<M: Message> Clone for MessageSender<M> {
    fn clone(&self) -> Self {
        Self {
            senders: Arc::clone(&self.senders),
        }
    }
}

impl<M: Message> MessageSender<M> {
    pub fn new(senders: Arc<(Vec<Sender<M>>, Vec<Sender<M>>)>) -> Self {
        Self { senders }
    }

    pub fn send(&self, message: M) {
        self.senders.0.iter().for_each(|s| {
            s.send(message.clone());
        });
        self.senders.1.iter().for_each(|s| {
            s.send(message.clone());
        });
    }

    pub fn send_to_update_thread(&self, message: M) {
        self.senders.1.iter().for_each(|s| {
            s.send(message.clone());
        });
    }

    pub fn send_to_render_thread(&self, message: M) {
        self.senders.0.iter().for_each(|s| {
            s.send(message.clone());
        });
    }
}
