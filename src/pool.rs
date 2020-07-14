mod thread;

use futures::future::BoxFuture;
use std::{error::Error, future::Future};

pub use thread::ThreadPool;

use amadeus_core::pool::ThreadPool as ThreadPoolTrait;

type Result<T> = std::result::Result<T, Box<dyn Error + Send>>;

impl ThreadPoolTrait for ThreadPool {
    fn spawn<F, Fut, T>(&self, work: F) -> BoxFuture<'static, Result<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + 'static,
        T: Send + 'static,
    {
        let _ = ThreadPool::spawn(self, work);
        todo!()
    }
}
