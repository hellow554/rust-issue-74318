#![type_length_limit = "13470730"]

use amadeus::prelude::{Cloudfront, ParallelStream, Source, ThreadPool};

#[tokio::test]
async fn cloudfront() {
    let _ = Cloudfront::new()
        .par_stream()
        .pipe_fork(&ThreadPool::new(), (), ())
        .await;
}
