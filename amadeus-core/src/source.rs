pub trait Source {
    type Item;
    type Error;

    type ParStream;
    type DistStream;

    fn par_stream(self) -> Self::ParStream;
    fn dist_stream(self) -> Self::DistStream;
}
