#![allow(non_snake_case, irrefutable_let_patterns)]

use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use sum::Sum2;

use super::{
    Factory, ParallelPipe, ParallelSink, PipeTask, PipeTaskAsync, Reducer, ReducerAsync,
    ReducerSend,
};
use crate::sink::Sink;

impl<Source, R0: ParallelSink<Source, Output = ()>, R1: ParallelSink<Source, Output = ()>>
    ParallelSink<Source> for (R0, R1)
where
    Source: Copy,
{
    type Output = ((), ());
    type Pipe = ((), ());
    type ReduceAFactory = ReduceA2Factory<(), ()>;
    type ReduceA = ReduceA2;
    type ReduceC = ReduceC2;

    fn reducers(self) -> (Self::Pipe, Self::ReduceAFactory, Self::ReduceC) {
        loop {}
    }
}

impl<Source, I0: ParallelPipe<Source>, I1: ParallelPipe<Source>> ParallelPipe<Source> for (I0, I1)
where
    Source: Copy,
{
    type Item = Sum2<I0::Item, I1::Item>;
    type Task = (I0::Task, I1::Task);

    fn task(&self) -> Self::Task {
        loop {}
    }
}

impl<Source, C0: PipeTask<Source>, C1: PipeTask<Source>> PipeTask<Source> for (C0, C1)
where
    Source: Copy,
{
    type Item = Sum2<C0::Item, C1::Item>;
    type Async = AsyncTuple2<Source, C0::Async, C1::Async>;
    fn into_async(self) -> Self::Async {
        loop {}
    }
}

pub struct AsyncTuple2<Source, C0, C1> {
    Copy: C0,
    B: C1,
    pending: Option<Option<Source>>,
}

impl<Source, C0: PipeTaskAsync<Source>, C1: PipeTaskAsync<Source>> PipeTaskAsync<Source>
    for AsyncTuple2<Source, C0, C1>
{
    type Item = Sum2<C0::Item, C1::Item>;

    #[allow(non_snake_case)]
    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        stream: Pin<&mut impl Stream<Item = Source>>,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        loop {}
    }
}

pub struct ReduceA2Factory<R0, R1>(pub(crate) R0, pub(crate) R1);
impl<R0: Factory, R1: Factory> Factory for ReduceA2Factory<R0, R1> {
    type Item = ReduceA2;

    fn make(&self) -> Self::Item {
        loop {}
    }
}

pub struct ReduceA2;
impl Reducer for ReduceA2 {
    type Item = Sum2<(), ()>;
    type Output = ((), ());
    type Async = ReduceA2Async;

    fn into_async(self) -> Self::Async {
        loop {}
    }
}

pub struct ReduceA2Async;

impl ReducerAsync for ReduceA2Async {
    type Item = Sum2<(), ()>;
    type Output = ((), ());

    fn poll_output(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {}
    }
}

impl ReducerSend for ReduceA2 {
    type Output = ((), ());
}

pub struct ReduceC2;
impl ReduceC2 {
    pub fn new() -> Self {
        loop {}
    }
}
impl Reducer for ReduceC2 {
    type Item = ((), ());
    type Output = ((), ());
    type Async = ReduceC2Async;

    fn into_async(self) -> Self::Async {
        loop {}
    }
}

pub struct ReduceC2Async;

impl ReducerAsync for ReduceC2Async {
    type Item = ((), ());
    type Output = ((), ());

    fn poll_output(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {}
    }
}
