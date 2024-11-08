pub trait Parser: Sized {
    fn parse(src: &str) -> crate::Result<Self>;
}
