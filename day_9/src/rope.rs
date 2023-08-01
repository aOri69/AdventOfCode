use std::collections::HashSet;

use crate::command::{Command, Direction};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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

#[derive(Debug)]
pub struct Rope {
    nodes: Vec<Position>,
    tail_visits: HashSet<Position>,
}

impl Default for Rope {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Rope {
    pub fn new(nodes: usize) -> Self {
        if nodes == 0 {
            return Self {
                nodes: vec![],
                tail_visits: HashSet::new(),
            };
        }

        let mut tail_visits = HashSet::new();
        tail_visits.insert(Position::default());

        Self {
            nodes: vec![Position::default(); nodes],
            tail_visits,
        }
    }

    pub fn process_command(&mut self, cmd: Command) {
        for _ in 0..cmd.steps() {}
        // advance head
        if let Some(head) = self.head_mut() {
            let steps = cmd.steps() as i32;
            match cmd.direction() {
                Direction::Up => *head += (0, steps).into(),
                Direction::Down => *head += (0, -steps).into(),
                Direction::Left => *head += (-steps, 0).into(),
                Direction::Right => *head += (steps, 0).into(),
            }
        }
        // advance tail

        // if let Some(tail) = self.nodes.last() {
        //     self.tail_visits.insert(*tail);
        // }
    }

    pub fn tail_visits_count(&self) -> usize {
        self.tail_visits.len()
    }

    fn head(&self) -> Option<&Position> {
        self.nodes.first()
    }

    fn head_mut(&mut self) -> Option<&mut Position> {
        self.nodes.first_mut()
    }

    fn tail(&self) -> Option<&Position> {
        self.nodes.last()
    }
}
