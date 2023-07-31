use std::collections::HashSet;

use crate::movement::{Movement, Position};

#[derive(Debug)]
pub struct Rope {
    head: Position,
    tail: Position,
    tail_visits: HashSet<Position>,
}

impl Rope {
    pub fn new() -> Self {
        let mut tail_visits = HashSet::new();
        tail_visits.insert(Position::default());
        Self {
            head: Position::default(),
            tail: Position::default(),
            tail_visits,
        }
    }

    pub fn process_movement(&mut self, movement: Movement) {
        for _ in 0..movement.steps() {
            // println!("H {} | T {}", self.head, self.tail);
            match movement {
                Movement::Up(_) => self.head.y += 1,
                Movement::Down(_) => self.head.y -= 1,
                Movement::Left(_) => self.head.x -= 1,
                Movement::Right(_) => self.head.x += 1,
            };
            self.advance_tail(&movement);
            // println!("after : H {} | T {}", self.head, self.tail);
        }
    }

    fn advance_tail(&mut self, movement: &Movement) {
        let y_delta = (self.head.y - self.tail.y).abs();
        let x_delta = (self.head.x - self.tail.x).abs();
        // Process tail movement only
        // if distance is more than 1 in any direction(2 is a diagonal move)
        if x_delta >= 2 || y_delta >= 2 {
            match movement {
                Movement::Up(_) => {
                    self.tail.y += 1;
                    if x_delta != 0 {
                        self.tail.x = self.head.x;
                    }
                }
                Movement::Down(_) => {
                    self.tail.y -= 1;
                    if x_delta != 0 {
                        self.tail.x = self.head.x;
                    }
                }
                Movement::Left(_) => {
                    self.tail.x -= 1;
                    if y_delta != 0 {
                        self.tail.y = self.head.y;
                    }
                }
                Movement::Right(_) => {
                    self.tail.x += 1;
                    if y_delta != 0 {
                        self.tail.y = self.head.y;
                    }
                }
            };
            self.tail_visits.insert(self.tail);
        }
    }

    pub fn tail_visits_count(&self) -> usize {
        self.tail_visits.len()
    }
}
