#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Item(u64);

impl Item {
    pub fn set(&mut self, v: u64) {
        self.0 = v;
    }
}

impl From<u64> for Item {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i64> for Item {
    fn from(value: i64) -> Self {
        Self(value as u64)
    }
}

impl From<i32> for Item {
    fn from(value: i32) -> Self {
        Self(value as u64)
    }
}

impl From<Item> for i64 {
    fn from(value: Item) -> Self {
        value.0 as i64
    }
}

impl From<Item> for u64 {
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
