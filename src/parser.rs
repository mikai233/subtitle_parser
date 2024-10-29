pub trait Parser: Sized {
    fn parse(src: &str) -> crate::error::Result<Self>;
}
