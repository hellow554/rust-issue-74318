use futures::{pin_mut, Stream};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::{ParallelStream, PipeTask, PipeTaskAsync, StreamTask, StreamTaskAsync};
use crate::sink::{Sink, SinkFlatMap};

pub struct FlatMap<I, F> {
    _i: I,
    _f: F,
}

impl_par_dist! {
    impl<I: ParallelStream, F, R: Stream> ParallelStream for FlatMap<I, F>
    where
        F: FnMut(I::Item) -> R + Send + 'static,
    {
        type Item = R::Item;
        type Task = FlatMapTask<I::Task, F>;
    }
}

pub struct FlatMapTask<C, F> {
    _task: C,
    _f: F,
}
impl<C: StreamTask, F: FnMut(C::Item) -> R, R: Stream> StreamTask for FlatMapTask<C, F> {
    type Item = R::Item;
    type Async = FlatMapStreamTaskAsync<C::Async, F, R>;
    fn into_async(self) -> Self::Async {
        loop {}
    }
}
impl<C: PipeTask<Source>, F: FnMut(C::Item) -> R, R: Stream, Source> PipeTask<Source>
    for FlatMapTask<C, F>
{
    type Item = R::Item;
    type Async = FlatMapStreamTaskAsync<C::Async, F, R>;
    fn into_async(self) -> Self::Async {
        loop {}
    }
}

#[pin_project]
pub struct FlatMapStreamTaskAsync<C, F, Fut> {
    #[pin]
    task: C,
    f: F,
    #[pin]
    fut: Option<Fut>,
}

impl<C: StreamTaskAsync, F: FnMut(C::Item) -> R, R: Stream> StreamTaskAsync
    for FlatMapStreamTaskAsync<C, F, R>
{
    type Item = R::Item;

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        let mut self_ = self.project();
        let (task, f) = (self_.task, &mut self_.f);
        let sink = SinkFlatMap::new(self_.fut, sink, |item| f(item));
        pin_mut!(sink);
        task.poll_run(cx, sink)
    }
}

impl<C: PipeTaskAsync<Source>, F, R: Stream, Source> PipeTaskAsync<Source>
    for FlatMapStreamTaskAsync<C, F, R>
where
    F: FnMut(<C as PipeTaskAsync<Source>>::Item) -> R,
{
    type Item = R::Item;

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        stream: Pin<&mut impl Stream<Item = Source>>,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        let mut self_ = self.project();
        let (task, f) = (self_.task, &mut self_.f);
        let sink = SinkFlatMap::new(self_.fut, sink, |item| f(item));
        pin_mut!(sink);
        task.poll_run(cx, stream, sink)
    }
}
