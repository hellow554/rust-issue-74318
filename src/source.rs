use futures::pin_mut;
use pin_project::pin_project;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::par_stream::{ParallelStream, StreamTask, StreamTaskAsync};

pub use amadeus_aws::Cloudfront;
pub mod aws {
    pub use crate::CloudfrontRow;
}

pub trait Source {
    type Item;
    type Error;

    type ParStream;
    type DistStream;

    fn par_stream(self) -> Self::ParStream;
    fn dist_stream(self) -> Self::DistStream;
}

impl Source for Cloudfront {
    type Item = crate::CloudfrontRow;
    type Error = ();

    type ParStream = IntoStream<<Self as amadeus_core::Source>::ParStream, Self::Item>;
    type DistStream = IntoStream<<Self as amadeus_core::Source>::DistStream, Self::Item>;

    fn par_stream(self) -> Self::ParStream {
        loop {}
    }
    fn dist_stream(self) -> Self::DistStream {
        loop {}
    }
}

pub struct IntoStream<I, U>(I, PhantomData<fn() -> U>);
impl<I, T, U> ParallelStream for IntoStream<I, U>
where
    I: ParallelStream<Item = Result<T, ()>>,
    T: Into<U>,
    U: 'static,
{
    type Item = Result<U, ()>;
    type Task = IntoTask<I::Task, U>;
}

#[pin_project]
pub struct IntoTask<I, U> {
    #[pin]
    task: I,
    marker: PhantomData<fn() -> U>,
}
impl<I, T, U> StreamTask for IntoTask<I, U>
where
    I: StreamTask<Item = Result<T, ()>>,
    T: Into<U>,
{
    type Item = Result<U, ()>;
    type Async = IntoTask<I::Async, U>;
    fn into_async(self) -> Self::Async {
        loop {}
    }
}
impl<I, T, U> StreamTaskAsync for IntoTask<I, U>
where
    I: StreamTaskAsync<Item = Result<T, ()>>,
    T: Into<U>,
{
    type Item = Result<U, ()>;

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        sink: Pin<&mut impl amadeus_core::sink::Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        let sink = amadeus_core::sink::SinkMap::new(sink, |item| panic!());
        pin_mut!(sink);
        self.project().task.poll_run(cx, sink)
    }
}
impl<I, T, E, U> Iterator for IntoStream<I, U>
where
    I: Iterator<Item = Result<T, E>>,
    T: Into<U>,
{
    type Item = Result<U, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
