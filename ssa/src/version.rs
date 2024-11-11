#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Version {
    V4,
    #[default]
    V4Plus,
}
