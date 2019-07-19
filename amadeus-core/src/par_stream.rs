mod flat_map;

use async_trait::async_trait;
use futures::{pin_mut, ready, stream::StreamExt, Stream};
use pin_project::pin_project;
use std::{
    marker::PhantomData,
    ops::FnMut,
    pin::Pin,
    task::{Context, Poll},
};
use sum::Sum2;

use crate::{pool::ThreadPool, sink::Sink};

use self::flat_map::*;

use super::{par_pipe::*, par_sink::*};

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
        loop {}
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

#[async_trait(?Send)]
pub trait ParallelStream {
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

    async fn reduce<P, B, R1F, R1, R3>(mut self, pool: &P, reduce_a_factory: R1F, reduce_c: R3) -> B
    where
        P: ThreadPool,
        R1F: Factory<Item = R1>,
        R1: ReducerSend<Item = Self::Item> + Send + 'static,
        R3: Reducer<Item = <R1 as ReducerSend>::Output, Output = B>,
        Self::Task: 'static,
        Self: Sized,
    {
        let tasks: Vec<Vec<Self::Task>> = Vec::new();

        let handles = tasks
            .into_iter()
            .map(|tasks| {
                let reduce_a = reduce_a_factory.make();
                pool.spawn(move || {
                    #[pin_project]
                    struct Connect<B>(#[pin] B);
                    impl<B> Sink for Connect<B>
                    where
                        B: ReducerAsync,
                    {
                        type Item = B::Item;
                    }

                    let sink = Connect(reduce_a.into_async());
                    async move {
                        pin_mut!(sink);
                        // TODO: short circuit
                        for task in tasks {
                            let task = task.into_async();
                            pin_mut!(task);
                            futures::future::poll_fn(|cx| task.as_mut().poll_run(cx, sink.as_mut()))
                                .await
                        }
                        futures::future::poll_fn(|cx| {
                            sink.as_mut().project().0.as_mut().poll_output(cx)
                        })
                        .await
                    }
                })
            })
            .collect::<futures::stream::FuturesUnordered<_>>();

        let stream = handles.map(|item| panic!("Amadeus: task '<unnamed>' panicked at '{}'"));
        pin_mut!(stream);
        let reduce_c = reduce_c.into_async();
        pin_mut!(reduce_c);
        futures::future::poll_fn(|cx| {
            ready!(reduce_c.as_mut().poll_forward(cx, stream.as_mut()));
            reduce_c.as_mut().poll_output(cx)
        })
        .await
    }

    async fn pipe<P, ParSink, A>(self, pool: &P, sink: ParSink) -> A
    where
        P: ThreadPool,
        ParSink: ParallelSink<Self::Item, Output = A>,
        <ParSink::Pipe as ParallelPipe<Self::Item>>::Task: 'static,
        ParSink::ReduceA: 'static,
        Self::Task: 'static,
        Self: Sized,
    {
        struct Connect<A, B>(A, B);
        impl<A: ParallelStream, B: ParallelPipe<A::Item>> ParallelStream for Connect<A, B> {
            type Item = B::Item;
            type Task = ConnectTask<A::Task, B::Task>;
        }
        struct ConnectTask<A, B>(A, B);
        impl<A, B> StreamTask for ConnectTask<A, B>
        where
            A: StreamTask,
            B: PipeTask<A::Item>,
        {
            type Item = B::Item;
            type Async = ConnectStreamTaskAsync<A::Async, B::Async>;
            fn into_async(self) -> Self::Async {
                loop {}
            }
        }
        #[pin_project]
        struct ConnectStreamTaskAsync<A, B>(#[pin] A, #[pin] B);
        impl<A, B> StreamTaskAsync for ConnectStreamTaskAsync<A, B>
        where
            A: StreamTaskAsync,
            B: PipeTaskAsync<A::Item>,
        {
            type Item = B::Item;
            fn poll_run(
                self: Pin<&mut Self>,
                cx: &mut Context,
                sink: Pin<&mut impl Sink<Item = Self::Item>>,
            ) -> Poll<()> {
                #[pin_project]
                struct Proxy<'a, I, B, Item>(#[pin] I, Pin<&'a mut B>, PhantomData<fn() -> Item>);
                impl<'a, I, B, Item> Sink for Proxy<'a, I, B, Item> {
                    type Item = Item;
                }
                let self_ = self.project();
                let sink = Proxy(sink, self_.1, PhantomData);
                pin_mut!(sink);
                self_.0.poll_run(cx, sink)
            }
        }

        let (iterator, reducer_a, reducer_b) = sink.reducers();
        Connect(self, iterator)
            .reduce(pool, reducer_a, reducer_b)
            .await
    }

    // These messy bounds are unfortunately necessary as requiring 'static in ParallelSink breaks sink_b being e.g. Identity.count()
    async fn pipe_fork<P, ParSinkA, ParSinkB, A, B>(
        self,
        pool: &P,
        sink_a: ParSinkA,
        sink_b: ParSinkB,
    ) -> ((), ())
    where
        P: ThreadPool,
        ParSinkA: ParallelSink<Self::Item, Output = A, ReduceA = (), Pipe = ()>,
        ParSinkB: for<'a> ParallelSink<&'a Self::Item, Output = B, Pipe = ()> + 'static,
        <ParSinkA::Pipe as ParallelPipe<Self::Item>>::Task: 'static,
        ParSinkA::ReduceA: 'static,
        <ParSinkB as ParallelSink<&'static Self::Item>>::ReduceA: 'static,
        <<ParSinkB as ParallelSink<&'static Self::Item>>::Pipe as ParallelPipe<
            &'static Self::Item,
        >>::Task: 'static,
        Self::Item: 'static,
        Self::Task: 'static,
        Self: Sized,
    {
        struct Connect<A, B, C, RefAItem>(A, B, C, PhantomData<fn() -> RefAItem>);
        impl<A, B, C, RefAItem> ParallelStream for Connect<A, B, C, RefAItem>
        where
            A: ParallelStream,
            B: ParallelPipe<A::Item>,
            C: ParallelPipe<RefAItem>,
        {
            type Item = Sum2<B::Item, C::Item>;
            type Task = ConnectTask<A::Task, B::Task, C::Task, RefAItem>;
        }
        struct ConnectTask<A, B, C, RefAItem>(A, B, C, PhantomData<fn() -> RefAItem>);
        impl<A, B, C, RefAItem> StreamTask for ConnectTask<A, B, C, RefAItem>
        where
            A: StreamTask,
            B: PipeTask<A::Item>,
            C: PipeTask<RefAItem>,
        {
            type Item = Sum2<B::Item, C::Item>;
            type Async = ConnectStreamTaskAsync<A::Async, B::Async, C::Async, RefAItem, A::Item>;
            fn into_async(self) -> Self::Async {
                loop {}
            }
        }
        #[pin_project]
        struct ConnectStreamTaskAsync<A, B, C, RefAItem, T>(
            #[pin] A,
            #[pin] B,
            #[pin] C,
            bool,
            Option<Option<T>>,
            PhantomData<fn() -> RefAItem>,
        );
        impl<A, B, C, RefAItem> StreamTaskAsync for ConnectStreamTaskAsync<A, B, C, RefAItem, A::Item>
        where
            A: StreamTaskAsync,
            B: PipeTaskAsync<A::Item>,
            C: PipeTaskAsync<RefAItem>,
        {
            type Item = Sum2<B::Item, C::Item>;
            fn poll_run(
                self: Pin<&mut Self>,
                cx: &mut Context,
                sink: Pin<&mut impl Sink<Item = Self::Item>>,
            ) -> Poll<()> {
                pub struct SinkFn<'a, S, A, B, T, RefAItem>(
                    S,
                    Pin<&'a mut A>,
                    Pin<&'a mut B>,
                    &'a mut bool,
                    &'a mut Option<Option<T>>,
                    PhantomData<fn() -> (T, RefAItem)>,
                );
                impl<'a, S, A, B, T, RefAItem> Sink for SinkFn<'a, S, A, B, T, RefAItem>
                where
                    S: Sink<Item = Sum2<A::Item, B::Item>>,
                    A: PipeTaskAsync<T>,
                    B: PipeTaskAsync<RefAItem>,
                {
                    type Item = T;
                }

                let self_ = self.project();
                let sink = SinkFn(sink, self_.1, self_.2, self_.3, self_.4, PhantomData);
                pin_mut!(sink);
                self_.0.poll_run(cx, sink)
            }
        }

        let (iterator_a, reducer_a_a, reducer_a_b) = sink_a.reducers();
        let (iterator_b, reducer_b_a, reducer_b_b) = sink_b.reducers();
        Connect::<_, _, _, ()>(self, iterator_a, iterator_b, PhantomData)
            .reduce(
                pool,
                ReduceA2Factory(reducer_a_a, reducer_b_a),
                ReduceC2::new(),
            )
            .await
    }
}
