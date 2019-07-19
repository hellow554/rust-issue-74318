use futures::{pin_mut, stream};
use std::{
    iter,
    pin::Pin,
    task::{Context, Poll},
};

use super::{DistributedStream, ParallelStream, StreamTask, StreamTaskAsync};
use crate::sink::Sink;

impl_par_dist_rename! {
    pub struct IterParStream<I>(pub(crate) I);

    impl<I: Iterator> ParallelStream for IterParStream<I> {
        type Item = ();
        type Task = IterStreamTask;
    }
}

pub struct IterStreamTask;

impl StreamTask for IterStreamTask {
    type Item = ();
    type Async = IterStreamTask;

    fn into_async(self) -> Self::Async {
        loop {}
    }
}
impl StreamTaskAsync for IterStreamTask {
    type Item = ();

    fn poll_run(
        self: Pin<&mut Self>,
        cx: &mut Context,
        sink: Pin<&mut impl Sink<Item = Self::Item>>,
    ) -> Poll<()> {
        let stream = stream::iter(iter::from_fn(|| loop {}));
        pin_mut!(stream);
        sink.poll_forward(cx, stream)
    }
}
