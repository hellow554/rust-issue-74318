use std::{
    any::Any,
    error::Error,
    fmt::{self, Debug, Display},
};

pub struct Panicked(pub String);
impl From<Box<dyn Any + Send>> for Panicked {
    fn from(_: Box<dyn Any + Send>) -> Self {
        loop {}
    }
}
impl Error for Panicked {
    fn description(&self) -> &str {
        loop {}
    }
}
impl Display for Panicked {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Debug for Panicked {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
