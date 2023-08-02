use std::collections::HashSet;

use crate::command::{Command, Direction};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
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
        const STEP: i32 = 1;
        // println!("{cmd:?}");
        for _ in 0..cmd.steps() {
            // advance head
            if let Some(head) = self.head_mut() {
                // let steps = cmd.steps() as i32;
                match cmd.direction() {
                    Direction::Up => *head += (0, STEP).into(),
                    Direction::Down => *head += (0, -STEP).into(),
                    Direction::Left => *head += (-STEP, 0).into(),
                    Direction::Right => *head += (STEP, 0).into(),
                }
            }
            // println!("HEAD - {:?}", self.head().unwrap());
            // println!("TAIL - {:?}", self.tail().unwrap());
            // advance tail
            // [1..] first is always a HEAD :)
            for i in 1..self.nodes.len() {
                //deltas between nearest nodes
                let (dx, dy) = (self.nodes[i - 1] - self.nodes[i]).into();
                // println!("{dx}-{dy}");
                let to_add: Position = match (dx, dy) {
                    // overlapping
                    (0, 0) => (0, 0).into(),
                    // one step nearby
                    (1, 0) | (0, 1) | (0, -1) | (-1, 0) => (0, 0).into(),
                    // diagonal nearby
                    (1, 1) | (-1, 1) | (1, -1) | (-1, -1) => (0, 0).into(),
                    // Up
                    (0, 2) => (0, 1).into(),
                    // Down
                    (0, -2) => (0, -1).into(),
                    // Left
                    (-2, 0) => (-1, 0).into(),
                    // Right
                    (2, 0) => (1, 0).into(),
                    // Up and Right
                    (1, 2) | (2, 1) => (1, 1).into(),
                    // Up and Left
                    (-2, 1) | (-1, 2) => (-1, 1).into(),
                    // Down and Right
                    (2, -1) | (1, -2) => (1, -1).into(),
                    // Down and Left
                    (-2, -1) | (-1, -2) => (-1, -1).into(),
                    // Diagonal
                    (-2, -2) => (-1, -1).into(),
                    (-2, 2) => (-1, 1).into(),
                    (2, -2) => (1, -1).into(),
                    (2, 2) => (1, 1).into(),
                    _ => panic!("not expected"),
                };
                // println!("TO_ADD: {to_add:?}");
                self.nodes[i] += to_add;

                if i == self.nodes.len() - 1 {
                    self.tail_visits.insert(self.nodes[i]);
                }
            }
        }
    }

    pub fn tail_visits_count(&self) -> usize {
        self.tail_visits.len()
    }

    pub fn head(&self) -> Option<&Position> {
        self.nodes.first()
    }

    fn head_mut(&mut self) -> Option<&mut Position> {
        self.nodes.first_mut()
    }

    pub fn tail_visits(&self) -> &HashSet<Position> {
        &self.tail_visits
    }
}
