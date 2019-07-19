use derive_new::new;
use futures::{pin_mut, ready, stream, Stream};
use pin_project::pin_project;
use std::{
    marker::PhantomData,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};

pub trait Sink {
    type Item;

    fn poll_forward(
        self: Pin<&mut Self>,
        cx: &mut Context,
        mut stream: Pin<&mut impl Stream<Item = Self::Item>>,
    ) -> Poll<()> {
        stream.as_mut().poll_next(cx);
        Poll::Ready(())
    }
}

impl<P> Sink for Pin<P>
where
    P: DerefMut + Unpin,
    P::Target: Sink,
{
    type Item = <P::Target as Sink>::Item;
}

impl Sink for () {
    type Item = ();
}

#[derive(new)]
pub struct SinkMap<F, I, R, Item> {
    _i: I,
    _f: F,
    marker: PhantomData<fn() -> (R, Item)>,
}
impl<F, I, R, Item> Sink for SinkMap<F, I, R, Item>
where
    F: FnMut(Item) -> R,
    I: Sink<Item = R>,
{
    type Item = Item;
}

#[pin_project]
#[derive(new)]
pub struct SinkFlatMap<'a, F, I, Fut, R, Item> {
    fut: Pin<&'a mut Option<Fut>>,
    #[pin]
    i: I,
    f: F,
    marker: PhantomData<fn() -> (Fut, R, Item)>,
}
impl<'a, F, I, R, Fut, Item> Sink for SinkFlatMap<'a, F, I, Fut, R, Item>
where
    F: FnMut(Item) -> Fut,
    Fut: Stream<Item = R>,
    I: Sink<Item = R>,
{
    type Item = Item;

    fn poll_forward(
        self: Pin<&mut Self>,
        cx: &mut Context,
        stream: Pin<&mut impl Stream<Item = Self::Item>>,
    ) -> Poll<()> {
        let self_ = self.project();
        let f = self_.f;
        let fut = self_.fut;
        let stream = stream::poll_fn(|cx| loop {
            let item = ready!(fut.as_mut().as_pin_mut().unwrap().poll_next(cx));
            if let Some(item) = item {
                break Poll::Ready(Some(item));
            }
            fut.set(None);
        });
        pin_mut!(stream);
        (self_.i).poll_forward(cx, stream)
    }
}
