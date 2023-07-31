use std::collections::HashSet;

use crate::command::Command;

#[derive(Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    head: Position,
    tail: Position,
    tail_visits: HashSet<Position>,
}

impl Rope {
    pub fn process_command(&mut self, cmd: Command) {
        todo!("process_command")
    }

    pub fn tail_visits_count(&self) -> usize {
        self.tail_visits.len()
    }
}
