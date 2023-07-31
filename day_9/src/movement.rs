use nom::{
    branch::alt,
    character::complete::{char, digit1, space1},
    combinator::{all_consuming, map_res},
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug)]
pub enum Movement {
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
    pub fn steps(&self) -> u32 {
        match self {
            Movement::Up(s) => *s,
            Movement::Down(s) => *s,
            Movement::Left(s) => *s,
            Movement::Right(s) => *s,
        }
    }
}

pub fn get_commands(input: &str) -> Vec<Movement> {
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
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.x, self.y)
    }
}
