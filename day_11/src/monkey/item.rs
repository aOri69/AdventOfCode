use super::{WorryLevel, WorryLevelUnsigned};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Item(WorryLevelUnsigned);

impl Item {
    pub fn set(&mut self, v: WorryLevelUnsigned) {
        self.0 = v;
    }
}

impl From<WorryLevelUnsigned> for Item {
    fn from(value: WorryLevelUnsigned) -> Self {
        Self(value)
    }
}

impl From<WorryLevel> for Item {
    fn from(value: WorryLevel) -> Self {
        Self(value as WorryLevelUnsigned)
    }
}

impl From<i32> for Item {
    fn from(value: i32) -> Self {
        Self(value as WorryLevelUnsigned)
    }
}

impl From<Item> for WorryLevel {
    fn from(value: Item) -> Self {
        value.0 as WorryLevel
    }
}

impl From<Item> for WorryLevelUnsigned {
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
