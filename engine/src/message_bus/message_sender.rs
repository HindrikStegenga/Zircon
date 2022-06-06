use super::*;
use asset_library::t_warn;
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
            if let Err(e) = s.send(message.clone()) {
                t_warn!("Could not send message: {}", e);
            }
        });
        self.senders.1.iter().for_each(|s| {
            if let Err(e) = s.send(message.clone()) {
                t_warn!("Could not send message: {}", e);
            }
        });
    }

    pub fn send_to_update_thread(&self, message: M) {
        self.senders.1.iter().for_each(|s| {
            if let Err(e) = s.send(message.clone()) {
                t_warn!("Could not send message: {}", e);
            }
        });
    }

    pub fn send_to_render_thread(&self, message: M) {
        self.senders.0.iter().for_each(|s| {
            if let Err(e) = s.send(message.clone()) {
                t_warn!("Could not send message: {}", e);
            }
        });
    }
}
