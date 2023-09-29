#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item(u32);

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
