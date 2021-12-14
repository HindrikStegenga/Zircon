use std::marker::PhantomData;
use super::*;

pub trait EngineEventHandler<E: EngineEvent> {
    fn on_event(&mut self, event: &E);
}


pub struct EventHandlerRegisterer<'a> {
    _0: PhantomData<&'a i32>
}