pub trait Message: Sized + Clone + Send + Sync + 'static {}
impl<T> Message for T where T : Sized + Clone + Send + Sync + 'static {}