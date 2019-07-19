use futures::future::BoxFuture;
use std::{error::Error, future::Future};

type Result<T> = std::result::Result<T, Box<dyn Error + Send>>;

pub trait ThreadPool {
    fn spawn<F, Fut, T>(&self, work: F) -> BoxFuture<'static, Result<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + 'static,
        T: Send + 'static;
}

impl<P: ?Sized> ThreadPool for &P
where
    P: ThreadPool,
{
    fn spawn<F, Fut, T>(&self, _: F) -> BoxFuture<'static, Result<T>> {
        loop {}
    }
}
