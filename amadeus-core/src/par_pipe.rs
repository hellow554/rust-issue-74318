use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::sink::Sink;

pub trait PipeTask<Source> {
    type Item;
    type Async: PipeTaskAsync<Source, Item = Self::Item>;

    fn into_async(self) -> Self::Async;
}

impl<T> PipeTask<T> for () {
    type Item = ();
    type Async = ();

    fn into_async(self) -> Self::Async {
        loop {}
    }
}

pub trait PipeTaskAsync<Source> {
    type Item;

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        stream: Pin<&mut impl Stream<Item = Source>>,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()>;
}

impl<T> PipeTaskAsync<T> for () {
    type Item = ();

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        stream: Pin<&mut impl Stream<Item = T>>,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        loop {}
    }
}

impl_par_dist_rename! {
    pub trait ParallelPipe<Source> {
        type Item;
        type Task: PipeTask<Source, Item = Self::Item> + Send;

        fn task(&self) -> Self::Task;
    }
}

impl<T> ParallelPipe<T> for () {
    type Item = ();
    type Task = ();

    fn task(&self) -> Self::Task {
        loop {}
    }
}
