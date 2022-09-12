use rayon_core::{Scope, ScopeFifo, ThreadPool, ThreadPoolBuilder};
use std::{future::Future, num::NonZeroUsize};
use tokio::{runtime::*, task::JoinHandle};

#[derive(Debug)]
pub struct Dispatcher {
    thread_pool: ThreadPool,
    runtime: Runtime,
}

impl Dispatcher {
    pub fn new(
        max_async_threads: Option<NonZeroUsize>,
        fallback_async_threads: NonZeroUsize,
        max_worker_thread: Option<NonZeroUsize>,
        fallback_worker_threads: NonZeroUsize,
    ) -> Option<Dispatcher> {
        let worker_threads = match max_worker_thread {
            Some(v) => v,
            None => std::thread::available_parallelism().unwrap_or(fallback_worker_threads),
        };

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(worker_threads.get())
            .build()
            .ok()?;

        let async_threads = match max_async_threads {
            Some(v) => v,
            None => fallback_async_threads,
        };

        let runtime = Builder::new_multi_thread()
            .thread_name("async")
            .worker_threads(async_threads.get())
            .max_blocking_threads(async_threads.get())
            .thread_stack_size(1024 * 1024 * 2)
            .build()
            .ok()?;

        Self {
            thread_pool,
            runtime,
        }
        .into()
    }
}

impl Dispatcher {
    pub fn spawn_async<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    // pub fn spawn_async_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    // where
    //     F: FnOnce() -> R + Send + 'static,
    //     R: Send + 'static,
    // {
    //     self.runtime.spawn_blocking(func)
    // }

    pub fn spawn_async_blocking<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    // #[inline(always)]
    // pub fn install<OP, R>(&self, op: OP) -> R
    // where
    //     OP: FnOnce() -> R + Send,
    //     R: Send,
    // {
    //     self.thread_pool.install(op)
    // }

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
