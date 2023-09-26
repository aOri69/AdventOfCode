mod parser;
mod utils;
use std::str::FromStr;

use nom::Finish;
pub use parser::*;

use self::{
    parser::parse_monkey,
    utils::{Item, Operation, Test},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn new(items: Items, operation: Operation, test: Test) -> Self {
        Self {
            items,
            operation,
            test,
        }
    }
}

impl FromStr for Monkey {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::error::Error;

        match parse_monkey(s).finish() {
            Ok((_remaining, monkey)) => Ok(monkey),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub type Monkeys = Vec<Monkey>;
pub type Items = Vec<Item>;
