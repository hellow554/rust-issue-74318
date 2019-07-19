use crate::par_stream::{DistributedStream, ParallelStream, StreamTask, StreamTaskAsync};

mod collections;
mod iterator;
use self::iterator::*;

impl_par_dist_rename! {
    pub trait IntoParallelStream {
        type ParStream;
        type Item;

        fn into_par_stream(self) -> Self::ParStream
        where
            Self: Sized;

            fn dist_stream_mut(&mut self) -> <&mut Self as IntoParallelStream>::ParStream
        where
            for<'a> &'a mut Self: IntoParallelStream,
        {
            loop {}
        }

        fn par_stream(&self) -> <&Self as IntoParallelStream>::ParStream
        where
            for<'a> &'a Self: IntoParallelStream,
        {
            loop {}
        }
    }
}
