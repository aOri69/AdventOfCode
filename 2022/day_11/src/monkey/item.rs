use std::ops::RemAssign;

use super::WorryLevel;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Item(WorryLevel);

impl Item {
    pub fn set(&mut self, v: WorryLevel) {
        self.0 = v;
    }
}

impl From<WorryLevel> for Item {
    fn from(value: WorryLevel) -> Self {
        Self(value)
    }
}

impl From<Item> for WorryLevel {
    fn from(value: Item) -> Self {
        value.0 as WorryLevel
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

impl RemAssign for Item {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}
