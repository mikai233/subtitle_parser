#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Version {
    V4,
    #[default]
    V4Plus,
}
