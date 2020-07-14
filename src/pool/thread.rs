use futures::{future, FutureExt, TryFutureExt};
use std::future::Future;

pub struct ThreadPool;
impl ThreadPool {
    pub fn spawn<F, Fut, T>(&self, work: F) -> impl Future<Output = Result<T, ()>> + Send
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn(DuckSend(future::lazy(|_| work()).flatten())).map_err(|_| ())
    }
}

use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
#[pin_project]
struct DuckSend<F>(#[pin] F);
impl<F> Future for DuckSend<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().0.poll(cx)
    }
}
unsafe impl<F> Send for DuckSend<F> {}
