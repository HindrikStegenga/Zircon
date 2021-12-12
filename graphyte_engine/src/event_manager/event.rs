
pub enum EventSourceThread {
    RenderThread,
    UpdateThread,
}

trait Event: Sized + Send + Sync + 'static {}
