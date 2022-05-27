mod async_task;

pub use async_task::*;
pub use rayon::prelude::*;
use std::sync::Arc;

use smol::{future::Future, Executor};
use AsyncTask;

use rayon::*;

#[derive(Debug)]
pub struct Dispatcher {
    thread_pool: rayon::ThreadPool,
    executor: Executor<'static>,
}

impl Dispatcher {
    pub fn new(max_worker_thread_count: Option<usize>) -> Dispatcher {
        let num_cpus = num_cpus::get();
        let worker_threads = match max_worker_thread_count {
            Some(v) => v,
            None => num_cpus - 1,
        };
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(worker_threads)
            .build()
            .unwrap();

        let executor = Executor::new();
        Self {
            thread_pool,
            executor,
        }
    }
}

unsafe impl Send for Dispatcher {}
unsafe impl Sync for Dispatcher {}

impl Dispatcher {
    pub fn tick_async_executor(self: &Arc<Self>) {
        let another_self = Arc::clone(&self);
        self.thread_pool
            .spawn(move || while another_self.executor.try_tick() {});
    }

    pub fn dispatch_async<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> AsyncTask<T> {
        let (sender, receiver) = smol::channel::bounded(1);
        self.executor
            .spawn(async move { sender.send(future.await).await })
            .detach();
        AsyncTask::new(receiver)
    }

    #[inline(always)]
    pub fn install<OP, R>(&self, op: OP) -> R
    where
        OP: FnOnce() -> R + Send,
        R: Send,
    {
        self.thread_pool.install(op)
    }

    #[inline(always)]
    pub fn join<A, B, RA, RB>(&self, oper_a: A, oper_b: B) -> (RA, RB)
    where
        A: FnOnce() -> RA + Send,
        B: FnOnce() -> RB + Send,
        RA: Send,
        RB: Send,
    {
        self.thread_pool.join(oper_a, oper_b)
    }

    #[inline(always)]
    pub fn scope<'scope, OP, R>(&self, op: OP) -> R
    where
        OP: for<'s> FnOnce(&'s Scope<'scope>) -> R + 'scope + Send,
        R: Send,
    {
        self.thread_pool.scope(op)
    }

    #[inline(always)]
    pub fn scope_fifo<'scope, OP, R>(&self, op: OP) -> R
    where
        OP: for<'s> FnOnce(&'s ScopeFifo<'scope>) -> R + 'scope + Send,
        R: Send,
    {
        self.thread_pool.scope_fifo(op)
    }

    #[inline(always)]
    pub fn spawn<OP>(&self, op: OP)
    where
        OP: FnOnce() + Send + 'static,
    {
        self.thread_pool.spawn(op)
    }

    #[inline(always)]
    pub fn spawn_fifo<OP>(&self, op: OP)
    where
        OP: FnOnce() + Send + 'static,
    {
        self.thread_pool.spawn_fifo(op)
    }
}