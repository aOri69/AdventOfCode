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

#[derive(Default)]
struct Position {
    x: u32,
    y: u32,
}

struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Position::default(),
            tail: Position::default(),
        }
    }

    fn process_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Up(steps) => {
                self.head.y += steps;
            }
            Movement::Down(steps) => {
                self.head.y -= steps;
            }
            Movement::Left(steps) => {
                self.head.x -= steps;
            }
            Movement::Right(steps) => {
                self.head.x += steps;
            }
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut rope = Rope::new();
    let commands = get_commands(input);

    commands
        .into_iter()
        .for_each(|cmd| rope.process_movement(cmd));

    todo!("part1")
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
