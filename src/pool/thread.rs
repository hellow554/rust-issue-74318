use futures::{future, FutureExt, TryFutureExt};
use std::future::Future;

use super::util::Panicked;

pub struct ThreadPool;
impl ThreadPool {
    pub fn new() -> Self {
        loop {}
    }

    pub fn spawn<F, Fut, T>(&self, work: F) -> impl Future<Output = Result<T, Panicked>> + Send
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + 'static,
        T: Send + 'static,
    {
        let _self = self;
        tokio::task::spawn(DuckSend(future::lazy(|_| work()).flatten()))
            .map_err(tokio::task::JoinError::into_panic)
            .map_err(Panicked::from)
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
#[allow(unsafe_code)]
unsafe impl<F> Send for DuckSend<F> {}
