#![feature(specialization)]
#![allow(unused_variables, incomplete_features)]

pub mod pool;
pub mod source;

pub struct CloudfrontRow;

impl From<amadeus_aws::CloudfrontRow> for CloudfrontRow {
    fn from(_: amadeus_aws::CloudfrontRow) -> Self {
        todo!()
    }
}

pub use amadeus_core::{par_pipe, par_stream};

pub use crate::{
    par_stream::{DistributedStream, ParallelStream},
    source::Source,
};

pub mod prelude {
    pub use super::{
        par_pipe::ParallelPipe, pool::ThreadPool, source::*, CloudfrontRow, ParallelStream,
    };
}

#[tokio::main]
async fn main() {
    let _ = prelude::Cloudfront
        .par_stream()
        .pipe_fork(&prelude::ThreadPool, (), ())
        .await;
}
