use super::{Factory, Reducer, ReducerSend};

impl Factory for () {
    type Item = ();

    fn make(&self) -> Self::Item {
        loop {}
    }
}

impl Reducer for () {
    type Item = ();
    type Output = ();
    type Async = ();

    fn into_async(self) -> Self::Async {
        loop {}
    }
}

impl ReducerSend for () {
    type Output = ();
}
