#![allow(unused_imports, dead_code, unused_variables)]

use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, opt, recognize, value},
    error::Error,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(i32),
}

impl FromStr for Command {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_command(s).finish() {
            Ok((_remaining, command)) => Ok(command),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    // prepare separate parsers as variables closures
    let noop_parser = value(Command::Noop, tag("noop"));
    let addx_parser = map(
        separated_pair(tag("addx"), tag(" "), nom::character::complete::i32),
        |(_addx, steps)| Command::Addx(steps),
    );
    // parse
    alt((noop_parser, addx_parser))(input)
}

pub fn sum_of_signal_strengths(input: &str) -> i32 {
    let commands = input
        .lines()
        .map(Command::from_str)
        .collect::<Result<Vec<_>, _>>();
    println!("{:?}", commands.unwrap());
    todo!("Part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input_zero() {
        use constants::TEST_SMALL;
        let result = sum_of_signal_strengths(TEST_SMALL);
        assert_eq!(result, 0i32);
    }

    #[test]
    fn large_input_non_zero() {
        use constants::{TEST_LARGE, TEST_SMALL};
        let result = sum_of_signal_strengths(TEST_LARGE);
        assert_eq!(result, 13140i32);
    }

    mod constants {
        pub const TEST_SMALL: &str = "noop
addx 3
addx -5
";

        pub const TEST_LARGE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    }
}
