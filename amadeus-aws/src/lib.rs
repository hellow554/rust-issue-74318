#![feature(type_alias_impl_trait)]
#![feature(never_type)]

use futures::{io::BufReader, AsyncBufReadExt, StreamExt};

use amadeus_core::{
    into_par_stream::IntoDistributedStream,
    par_stream::DistributedStream,
    util::{DistParStream, ResultExpandIter},
    Source,
};

pub struct Cloudfront;

impl Cloudfront {
    pub fn new() -> Self {
        loop {}
    }
}

impl Source for Cloudfront {
    type Item = CloudfrontRow;
    type Error = ();

    type ParStream =
        impl amadeus_core::par_stream::ParallelStream<Item = Result<Self::Item, Self::Error>>;
    type DistStream = impl DistributedStream<Item = Result<Self::Item, Self::Error>>;

    fn par_stream(self) -> Self::ParStream {
        DistParStream::new(self.dist_stream())
    }

    fn dist_stream(self) -> Self::DistStream {
        Vec::<()>::new().into_dist_stream().flat_map(move |_: ()| {
            ResultExpandIter::new(Ok(BufReader::new("".as_bytes())
                .lines()
                .map(|_| CloudfrontRow::from_line())))
        })
    }
}

pub struct CloudfrontRow;

impl CloudfrontRow {
    fn from_line() -> Self {
        loop {}
    }
}
