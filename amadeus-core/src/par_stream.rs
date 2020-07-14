mod flat_map;

use futures::{stream::StreamExt, Stream};
use std::{
    ops::FnMut,
    pin::Pin,
    task::{Context, Poll},
};

use crate::sink::Sink;

use self::flat_map::*;

use super::par_pipe::*;

pub trait StreamTask {
    type Item;
    type Async: StreamTaskAsync<Item = Self::Item>;
    fn into_async(self) -> Self::Async;
}

pub trait StreamTaskAsync {
    type Item;

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        todo!()
    }
}

impl StreamTask for () {
    type Item = ();
    type Async = ();
    fn into_async(self) -> Self::Async {}
}

impl StreamTaskAsync for () {
    type Item = ();
}

pub trait DistributedStream {
    type Item;
    type Task: StreamTask<Item = Self::Item> + Send;

    fn flat_map<B, F>(self, _: F) -> FlatMap<Self, F>
    where
        F: FnMut(Self::Item) -> B + Clone + Send + 'static,
        B: Stream,
        Self: Sized,
    {
        loop {}
    }
}

pub trait ParallelStream {
    type Item;
    type Task: StreamTask<Item = Self::Item> + Send;

    async fn reduce<P, B, R1F, R1, R3>(
        self,
    ) {
        let handles = vec![]
            .into_iter()
            .map(|tasks| {});
    }
}
