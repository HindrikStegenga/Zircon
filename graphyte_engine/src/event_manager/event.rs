
#[repr(u8)]
pub enum EventSourceThread {
    RenderThread,
    UpdateThread,
}
#[repr(u8)]
pub enum EventDestinationThread {
    AllThreads = 0,
    RenderThread = 1,
    UpdateThread = 2,
}

pub trait EngineEvent: Sized + Clone + Send + Sync + 'static {}
