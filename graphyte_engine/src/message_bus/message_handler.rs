use super::*;

pub trait MessageHandler<C, M: Message> {
    fn handle(&mut self, context: &mut C, message: M);
}

