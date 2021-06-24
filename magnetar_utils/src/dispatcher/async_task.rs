use smol::channel::{Receiver, TryRecvError};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AsyncTaskError {
    Empty,
    Closed,
}

impl From<TryRecvError> for AsyncTaskError {
    fn from(v: TryRecvError) -> Self {
        match v {
            TryRecvError::Empty => AsyncTaskError::Empty,
            TryRecvError::Closed => AsyncTaskError::Closed,
        }
    }
}

pub struct AsyncTask<T> {
    receiver: Receiver<T>,
}

impl<T> AsyncTask<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    pub fn cancel(&self) -> bool {
        self.receiver.close()
    }

    pub fn is_cancelled(&self) -> bool {
        self.receiver.is_closed()
    }

    pub fn poll(&self) -> Result<T, AsyncTaskError> {
        match self.receiver.try_recv() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wait(&self) -> Result<T, AsyncTaskError> {
        unimplemented!()
    }
}
