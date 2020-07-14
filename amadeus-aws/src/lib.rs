#![feature(type_alias_impl_trait)]

use futures::{io::BufReader, AsyncBufReadExt, StreamExt};

use amadeus_core::{
    into_par_stream::IntoDistributedStream,
    par_stream::DistributedStream,
    util::{DistParStream, ResultExpandIter},
    Source,
};

pub struct Cloudfront;

impl Source for Cloudfront {
    type Item = CloudfrontRow;
    type Error = ();

    type ParStream =
        impl amadeus_core::par_stream::ParallelStream<Item = Result<Self::Item, Self::Error>>;
    type DistStream = impl DistributedStream<Item = Result<Self::Item, Self::Error>>;

    fn par_stream(self) -> Self::ParStream {
        DistParStream(self.dist_stream())
    }

    fn dist_stream(self) -> Self::DistStream {
        Vec::<()>::new().into_dist_stream().flat_map(move |_: ()| {
            ResultExpandIter::Ok(
                BufReader::new("".as_bytes())
                    .lines()
                    .map(|_| CloudfrontRow::new()),
            ) // it's important to call a function here
        })
    }
}

pub struct CloudfrontRow;

impl CloudfrontRow {
    fn new() -> Self {
        todo!()
    }
}
