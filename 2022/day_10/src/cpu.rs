use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
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

fn parse_command(input: &str) -> IResult<&str, Instruction> {
    // prepare separate parsers as variables closures
    let noop_parser = value(Instruction::Noop, tag("noop"));
    let addx_parser = map(
        separated_pair(tag("addx"), tag(" "), nom::character::complete::i32),
        |(_addx, steps)| Instruction::Addx(steps),
    );
    // parse
    alt((noop_parser, addx_parser))(input)
}

pub struct Cpu {
    cycle: u32,
    reg_x: i32,
    current_command: Option<Instruction>,
    ticks_used: u32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            cycle: Default::default(),
            reg_x: 1,
            current_command: Default::default(),
            ticks_used: Default::default(),
        }
    }
}

impl Cpu {
    pub fn tick(&mut self) {
        self.cycle += 1;
        if let Some(command) = self.current_command {
            self.ticks_used += 1;
            if self.ticks_used == command.cycles() {
                match command {
                    Instruction::Noop => (),
                    Instruction::Addx(x) => self.reg_x += x,
                }
                self.set_command(None);
            }
        }
    }

    pub fn set_command(&mut self, command: Option<Instruction>) {
        self.ticks_used = 0;
        match command {
            Some(command) => self.current_command = Some(command),
            None => self.current_command = None,
        }
    }

    pub fn reg_x(&self) -> i32 {
        self.reg_x
    }

    pub fn cycle(&self) -> u32 {
        self.cycle
    }

    pub fn signal_strength(&self) -> i32 {
        self.cycle() as i32 * self.reg_x()
    }

    pub fn current_command(&self) -> Option<Instruction> {
        self.current_command
    }
}
