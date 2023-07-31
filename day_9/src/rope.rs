use std::collections::HashSet;

use crate::command::Command;

#[derive(Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
