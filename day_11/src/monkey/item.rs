use std::fmt::write;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Item(u32);

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Item> for i32 {
    fn from(value: Item) -> Self {
        value.0 as i32
    }
}

impl From<Item> for u32 {
    fn from(value: Item) -> Self {
        value.0
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
