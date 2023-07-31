mod error;

use std::str::FromStr;

use crate::instruction::error::CommandError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, value},
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    steps: u32,
}

impl FromStr for Command {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Command::parse_command(s).finish() {
            Ok((_remaining, command)) => Ok(command),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Command {
    pub fn get_commands(input: &str) -> Result<Vec<Command>, nom::error::Error<String>> {
        input.lines().map(Command::from_str).collect()
    }

    fn parse_steps(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    fn parse_command(input: &str) -> IResult<&str, Command> {
        map(
            separated_pair(Direction::parse_direction, tag(" "), Command::parse_steps),
            |(direction, steps)| Self { direction, steps },
        )(input)
    }
}
