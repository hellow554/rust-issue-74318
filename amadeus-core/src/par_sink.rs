mod folder;
mod tuple;

use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::par_pipe::*;

pub use self::tuple::*;

pub trait Reducer {
    type Item;
    type Output;
    type Async: ReducerAsync<Item = Self::Item, Output = Self::Output>;

    fn into_async(self) -> Self::Async;
}

pub trait ReducerAsync {
    type Item;
    type Output;

    fn poll_forward(
        self: Pin<&mut Self>,
        cx: &mut Context,
        mut stream: Pin<&mut impl Stream<Item = Self::Item>>,
    ) -> Poll<()> {
        stream.as_mut().poll_next(cx);
        Poll::Ready(())
    }
    fn poll_output(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

impl ReducerAsync for () {
    type Item = ();
    type Output = ();

    fn poll_output(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {}
    }
}

pub trait ReducerSend: Reducer<Output = <Self as ReducerSend>::Output> {
    type Output: Send + 'static;
}
pub trait ReducerProcessSend: ReducerSend<Output = <Self as ReducerProcessSend>::Output> {
    type Output: Send + 'static;
}

pub trait Factory {
    type Item;

    fn make(&self) -> Self::Item;
}

pub trait DistributedSink<Source> {
    type Output;
    type Pipe: DistributedPipe<Source>;
    type ReduceAFactory: Factory<Item = Self::ReduceA> + Send;
    type ReduceBFactory: Factory<Item = Self::ReduceB>;
    type ReduceA: ReducerSend<Item = <Self::Pipe as DistributedPipe<Source>>::Item> + Send;
    type ReduceB: ReducerProcessSend<Item = <Self::ReduceA as Reducer>::Output> + Send;
    type ReduceC: Reducer<Item = <Self::ReduceB as Reducer>::Output, Output = Self::Output>;

    fn reducers(
        self,
    ) -> (
        Self::Pipe,
        Self::ReduceAFactory,
        Self::ReduceBFactory,
        Self::ReduceC,
    );
}

pub trait ParallelSink<Source> {
    type Output;
    type Pipe: ParallelPipe<Source>;
    type ReduceAFactory: Factory<Item = Self::ReduceA>;
    type ReduceA: ReducerSend<Item = <Self::Pipe as ParallelPipe<Source>>::Item> + Send;
    type ReduceC: Reducer<Item = <Self::ReduceA as Reducer>::Output, Output = Self::Output>;

    fn reducers(self) -> (Self::Pipe, Self::ReduceAFactory, Self::ReduceC);
}

impl<T> ParallelSink<T> for () {
    type Output = ();
    type Pipe = ();
    type ReduceAFactory = ();
    type ReduceA = ();
    type ReduceC = ();

    fn reducers(self) -> (Self::Pipe, Self::ReduceAFactory, Self::ReduceC) {
        loop {}
    }
}
