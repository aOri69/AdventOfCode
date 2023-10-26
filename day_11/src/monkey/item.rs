#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Item(u32);

impl Item {
    pub fn set(&mut self, v: u32) {
        self.0 = v;
    }
}

impl From<u32> for Item {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for Item {
    fn from(value: i32) -> Self {
        Self(value as u32)
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
