pub trait Message: Sized + Clone + Send + 'static {}
impl<T> Message for T where T: Sized + Clone + Send + 'static {}
