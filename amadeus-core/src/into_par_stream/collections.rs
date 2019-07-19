use std::vec;

use super::{IntoDistributedStream, IntoParallelStream, IterDistStream, IterParStream};

impl_par_dist_rename! {
    impl<T> IntoParallelStream for Vec<T> {
        type ParStream = IterParStream<vec::IntoIter<T>>;
        type Item = T;

        fn into_par_stream(self) -> Self::ParStream {
            loop {}
        }
    }
}
