pub use builder_derive::Builder;

pub trait Builder {
    fn new() -> Self;
}
