use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, digit1, space1},
    combinator::{all_consuming, map_res},
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl TryFrom<(char, u32)> for Movement {
    type Error = String;

    fn try_from(value: (char, u32)) -> Result<Self, Self::Error> {
        match value.0 {
            'U' => Ok(Self::Up(value.1)),
            'D' => Ok(Self::Down(value.1)),
            'L' => Ok(Self::Left(value.1)),
            'R' => Ok(Self::Right(value.1)),
            _ => Err(format!("Unable to create Movement from {:?}", value)),
        }
    }
}

impl Movement {
    fn steps(&self) -> u32 {
        match self {
            Movement::Up(s) => *s,
            Movement::Down(s) => *s,
            Movement::Left(s) => *s,
            Movement::Right(s) => *s,
        }
    }
}

fn get_commands(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|l| all_consuming(parse_movement)(l).finish().unwrap().1)
        .collect()
}

fn int_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_movement(input: &str) -> IResult<&str, Movement> {
    map_res(
        separated_pair(
            alt((char('U'), char('D'), char('L'), char('R'))),
            space1,
            int_parser,
        ),
        |(direction, steps)| Movement::try_from((direction, steps)),
    )(input)
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
    tail_visits: HashSet<Position>,
}

impl Rope {
    fn new() -> Self {
        let mut tail_visits = HashSet::new();
        tail_visits.insert(Position::default());
        Self {
            head: Position::default(),
            tail: Position::default(),
            tail_visits,
        }
    }

    fn process_movement(&mut self, movement: Movement) {
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

    fn tail_visits_count(&self) -> usize {
        self.tail_visits.len()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut rope = Rope::new();
    let commands = get_commands(input);

    commands
        .into_iter()
        .for_each(|cmd| rope.process_movement(cmd));
    // dbg!(&rope);
    rope.tail_visits_count()
    // todo!("part1")
}

#[cfg(test)]
mod tests {
    pub use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13);
    }
}
