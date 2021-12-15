use super::*;

pub trait MessageHandler<M: Message> {
    fn handle(&mut self, message: M);
}