mod parser;
mod utils;
use std::{rc::Rc, str::FromStr};

pub use self::utils::{Item, Operation, Test};

pub struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
        // match parse_monkey(s).finish() {
        //     Ok((_remaining, monkey)) => Ok(monkey),
        //     Err(e) => Err("Error"),
        // }
    }
}

pub type Monkeys = Vec<Monkey>;
pub type Items = Vec<Item>;
